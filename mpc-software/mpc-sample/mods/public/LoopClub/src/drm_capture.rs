//! QA-only readback of the active DRM CRTC. No modesetting, framebuffer writes,
//! master takeover, or kernel changes. ABI: Linux drm.h / drm_mode.h v6.1.
//! Unlike fbdev, this follows the application's currently scanned-out framebuffer.
#[cfg(target_os = "linux")]
mod linux {
    use std::{fs::OpenOptions, os::fd::AsRawFd};
    #[repr(C)]
    #[derive(Default)]
    struct Resources {
        pointers: [u64; 4],
        counts: [u32; 4],
        limits: [u32; 4],
    }
    #[repr(C)]
    struct Crtc {
        connectors: u64,
        fields: [u32; 7],
        mode: [u8; 68],
    }
    #[repr(C)]
    #[derive(Default)]
    struct Frame {
        id: u32,
        width: u32,
        height: u32,
        pitch: u32,
        bpp: u32,
        depth: u32,
        handle: u32,
    }
    #[repr(C)]
    struct Map {
        handle: u32,
        pad: u32,
        offset: u64,
    }
    unsafe extern "C" {
        fn ioctl(fd: i32, request: usize, ...) -> i32;
        fn mmap(
            address: *mut core::ffi::c_void,
            length: usize,
            protection: i32,
            flags: i32,
            fd: i32,
            offset: i64,
        ) -> *mut core::ffi::c_void;
        fn munmap(address: *mut core::ffi::c_void, length: usize) -> i32;
    }
    fn get<T>(fd: i32, number: usize, value: &mut T) -> Result<(), String> {
        let request = 0xc0000000usize | (std::mem::size_of::<T>() << 16) | (0x64 << 8) | number;
        // SAFETY: each caller supplies the matching Linux UAPI repr(C) structure;
        // ioctl completes synchronously and retains no userspace pointer.
        if unsafe { ioctl(fd, request, value as *mut T) } < 0 {
            Err(format!(
                "DRM ioctl {number:x}: {}",
                std::io::Error::last_os_error()
            ))
        } else {
            Ok(())
        }
    }
    struct Handle {
        fd: i32,
        handle: u32,
    }
    impl Drop for Handle {
        fn drop(&mut self) {
            let mut args = [self.handle, 0u32];
            // SAFETY: release only this new GETFB reference on our own descriptor.
            // The application's separate handle and framebuffer remain untouched.
            unsafe {
                ioctl(self.fd, 0x40086409usize, args.as_mut_ptr());
            }
        }
    }
    pub fn screen() -> Result<Vec<u8>, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/dri/card0")
            .map_err(|e| e.to_string())?;
        let fd = file.as_raw_fd();
        let mut resources = Resources::default();
        get(fd, 0xa0, &mut resources)?;
        if resources.counts[1] == 0 || resources.counts[1] > 16 {
            return Err("Unexpected CRTC count".into());
        }
        let mut ids = [0u32; 16];
        resources.pointers[1] = ids.as_mut_ptr() as u64;
        resources.counts[0] = 0;
        resources.counts[2] = 0;
        resources.counts[3] = 0;
        resources.counts[1] = ids.len() as u32;
        get(fd, 0xa0, &mut resources)?;
        if resources.counts[1] > ids.len() as u32 {
            return Err("CRTC list changed".into());
        }
        for id in ids.iter().take(resources.counts[1] as usize) {
            let mut crtc = Crtc {
                connectors: 0,
                fields: [0; 7],
                mode: [0; 68],
            };
            crtc.fields[1] = *id;
            get(fd, 0xa1, &mut crtc)?;
            if crtc.fields[2] == 0 || crtc.fields[6] == 0 {
                continue;
            }
            if crtc.fields[3] != 0 || crtc.fields[4] != 0 {
                return Err("Unsupported CRTC crop".into());
            }
            let mut frame = Frame {
                id: crtc.fields[2],
                ..Frame::default()
            };
            get(fd, 0xad, &mut frame)?;
            if frame.handle == 0 {
                return Err("GETFB did not grant a readback handle".into());
            }
            let _handle = Handle {
                fd,
                handle: frame.handle,
            };
            if frame.width != 320
                || frame.height != 240
                || frame.bpp != 32
                || frame.depth != 24
                || frame.pitch < 1280
                || frame.pitch > 8192
            {
                return Err("Unsupported active framebuffer format".into());
            }
            let mut map = Map {
                handle: frame.handle,
                pad: 0,
                offset: 0,
            };
            get(fd, 0xb3, &mut map)?;
            if map.offset > i64::MAX as u64 {
                return Err("Invalid DRM map offset".into());
            }
            let length = frame.pitch as usize * 240;
            // SAFETY: a validated dumb-buffer handle returned this offset. Map
            // read-only; copy bounded bytes while file and GETFB handle are alive.
            let address =
                unsafe { mmap(std::ptr::null_mut(), length, 1, 1, fd, map.offset as i64) };
            if address as isize == -1 {
                return Err(std::io::Error::last_os_error().to_string());
            }
            let mut bytes = vec![0u8; length];
            // SAFETY: mapping is readable for exactly length bytes. The native
            // app may update pixels concurrently, so a capture can contain tearing.
            unsafe {
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = std::ptr::read_volatile((address as *const u8).add(i));
                }
                munmap(address, length);
            }
            return crate::qa::bmp(&bytes, 320, 240, frame.pitch as usize, 32)
                .map_err(String::from);
        }
        Err("No active DRM framebuffer".into())
    }
    const _: () = {
        assert!(std::mem::size_of::<Resources>() == 64);
        assert!(std::mem::size_of::<Crtc>() == 104);
        assert!(std::mem::size_of::<Frame>() == 28);
        assert!(std::mem::size_of::<Map>() == 16);
    };
    #[test]
    fn linux_uapi_layouts() {
        assert_eq!(std::mem::size_of::<Resources>(), 64);
        assert_eq!(std::mem::size_of::<Crtc>(), 104);
        assert_eq!(std::mem::size_of::<Frame>(), 28);
        assert_eq!(std::mem::size_of::<Map>(), 16);
    }
}
pub fn screen() -> Result<Vec<u8>, String> {
    #[cfg(target_os = "linux")]
    {
        linux::screen()
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err("Actual DRM readback requires MPC Sample Linux".into())
    }
}
