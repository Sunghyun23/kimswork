//! Constrained development persistence: verified direct Loop Club autorun only.
//! No arbitrary paths, firmware writes or owner-session writes. Entire old boot
//! directory is retained on the same card before atomically publishing the new one.
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const SALT: &[u8] = b"f7bed1c9-7e90-4731-9194-c996666b4331";
const ALLOWED: &[&str] = &[
    "INSTALL.txt",
    "LICENSE",
    "SDK-LICENSE",
    "autorun.check",
    "autorun.json",
    "autorun.sh",
    "compatibility.json",
    "loop-club-qa-agent",
    "loop-club.so",
    "sampleboot.txt",
];
fn error(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn hash(path: &Path) -> Result<String, String> {
    let out = Command::new("sha256sum")
        .env_remove("LD_PRELOAD")
        .arg(path)
        .output()
        .map_err(error)?;
    if !out.status.success() {
        return Err("SHA-256 failed".into());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let sha = text.split_whitespace().next().unwrap_or("");
    if sha.len() != 64 || !sha.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("Invalid SHA-256 output".into());
    }
    Ok(sha.into())
}
fn hashes(root: &Path) -> Result<BTreeMap<String, String>, String> {
    if fs::symlink_metadata(root)
        .map_err(error)?
        .file_type()
        .is_symlink()
    {
        return Err("Boot symlink refused".into());
    }
    let mut result = BTreeMap::new();
    for item in fs::read_dir(root).map_err(error)? {
        let item = item.map_err(error)?;
        let name = item
            .file_name()
            .into_string()
            .map_err(|_| "Non-UTF8 boot filename")?;
        if !item.file_type().map_err(error)?.is_file()
            || !(ALLOWED.contains(&name.as_str()) || name.starts_with("._"))
        {
            return Err("Expected Loop Club-only flat boot folder".into());
        }
        if item.metadata().map_err(error)?.len() > 8 * 1024 * 1024 {
            return Err("Oversize boot file".into());
        }
        result.insert(name, hash(&item.path())?);
    }
    Ok(result)
}
fn metadata(root: &Path) -> Result<serde_json::Value, String> {
    let active = root.join("autorun");
    let values = hashes(&active)?;
    let meta: serde_json::Value =
        serde_json::from_slice(&fs::read(active.join("compatibility.json")).map_err(error)?)
            .map_err(error)?;
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(active.join("autorun.json")).map_err(error)?)
            .map_err(error)?;
    if meta["product"] != "Loop Club"
        || meta["boot_mode"] != "direct"
        || manifest["value0"]["scripts"][0]["askQuestion"] != false
        || manifest["value0"]["scripts"][0]["path"] != "autorun.sh"
    {
        return Err("Expected existing direct Loop Club boot".into());
    }
    if meta["library_sha256"].as_str() != values.get("loop-club.so").map(String::as_str) {
        return Err("Existing boot library SHA mismatch".into());
    }
    Ok(meta)
}
pub fn device_root() -> Result<PathBuf, String> {
    let root = std::env::var_os("LOOP_CLUB_CARD_DIR")
        .map(PathBuf::from)
        .ok_or("No card root")?
        .canonicalize()
        .map_err(error)?;
    if root == Path::new("/") {
        return Err("Invalid card root".into());
    }
    let mounts = fs::read_to_string("/proc/mounts").map_err(error)?;
    let mounted = mounts.lines().any(|line| {
        let p: Vec<_> = line.split_whitespace().collect();
        p.len() > 2
            && ["exfat", "vfat", "fuseblk"].contains(&p[2])
            && Path::new(&p[1].replace("\\040", " ")) == root
    });
    if !mounted {
        return Err("Card is not mounted locally; leave SD export first".into());
    }
    Ok(root)
}
pub fn inspect() -> Result<serde_json::Value, String> {
    let root = device_root()?;
    let meta = metadata(&root)?;
    Ok(
        serde_json::json!({"card":root,"product":meta["product"],"boot_mode":"direct","previous_library_sha256":meta["library_sha256"],"runtime_sha256":hash(Path::new("/run/loop-club.so"))?,"scope":"only direct Loop Club autorun; projects/samples/expansions untouched"}),
    )
}
pub fn publish(
    root: &Path,
    runtime: &Path,
    previous: &str,
    expected: &str,
) -> Result<serde_json::Value, String> {
    let mut meta = metadata(root)?;
    let old = root.join("autorun");
    let old_hashes = hashes(&old)?;
    if meta["library_sha256"] != previous || hash(runtime)? != expected {
        return Err("Boot/runtime identity changed; inspect again".into());
    }
    let runtime_bytes = fs::read(runtime).map_err(error)?;
    if runtime_bytes.len() < 64
        || runtime_bytes.len() > 8 * 1024 * 1024
        || !runtime_bytes.starts_with(b"\x7fELF\x02\x01\x01")
    {
        return Err("Invalid runtime library".into());
    }
    let mut script = fs::read_to_string(old.join("autorun.sh")).map_err(error)?;
    let old_guard = format!(
        "[ \"$(sha256sum \"$DIR/loop-club.so\" | cut -d\" \" -f1)\" = \"{previous}\" ] || exit 0"
    );
    if script.matches(&old_guard).count() != 1 {
        return Err("Unrecognized library integrity guard".into());
    }
    script = script.replace(&old_guard, &old_guard.replace(previous, expected));
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(error)?
        .as_nanos();
    let name = format!("remote-{}-{stamp}", std::process::id());
    let incoming = root.join(format!(".loopclub-{name}"));
    let backup = root.join(".LoopClubBackups").join(&name);
    fs::create_dir_all(&backup).map_err(error)?;
    fs::create_dir(&incoming).map_err(error)?;
    for file in old_hashes.keys() {
        fs::copy(old.join(file), incoming.join(file)).map_err(error)?;
    }
    fs::write(incoming.join("loop-club.so"), runtime_bytes).map_err(error)?;
    fs::write(incoming.join("autorun.sh"), script).map_err(error)?;
    meta["library_sha256"] = expected.into();
    meta["sdk_build_patch_sha256"] = option_env!("LOOP_CLUB_SDK_PATCH")
        .map(serde_json::Value::from)
        .unwrap_or(serde_json::Value::Null);
    meta["qa_variant"] = crate::qa::build_id().into();
    meta["device_installed"] = true.into();
    meta["screen_capture"] = "active DRM actual-device QA".into();
    fs::write(
        incoming.join("compatibility.json"),
        serde_json::to_vec_pretty(&meta).map_err(error)?,
    )
    .map_err(error)?;
    if incoming.join("sampleboot.txt").exists() {
        fs::write(
            incoming.join("sampleboot.txt"),
            format!("Loop Club USB {}\n", crate::qa::build_id()),
        )
        .map_err(error)?;
    }
    let mut check = fs::read(incoming.join("autorun.json")).map_err(error)?;
    check.extend(fs::read(incoming.join("autorun.sh")).map_err(error)?);
    check.extend(SALT);
    let check_file = backup.join("check-input.bin");
    fs::write(&check_file, check).map_err(error)?;
    let digest = hash(&check_file)?;
    fs::write(incoming.join("autorun.check"), digest).map_err(error)?;
    let new_hashes = hashes(&incoming)?;
    if new_hashes.get("loop-club.so").map(String::as_str) != Some(expected) {
        return Err("Staged boot read-back mismatch".into());
    }
    for file in new_hashes.keys() {
        fs::File::open(incoming.join(file))
            .and_then(|f| f.sync_all())
            .map_err(error)?;
    }
    fs::write(backup.join("manifest.json"),serde_json::to_vec_pretty(&serde_json::json!({"old_sha256":old_hashes,"new_sha256":new_hashes,"restore":"Move active autorun aside; rename autorun-original back to card root"})).map_err(error)?).map_err(error)?;
    fs::rename(&old, backup.join("autorun-original")).map_err(error)?;
    let activated = (|| {
        if hashes(&backup.join("autorun-original"))? != old_hashes {
            return Err("Moved original mismatch".into());
        }
        fs::rename(&incoming, &old).map_err(error)?;
        if hashes(&old)? != new_hashes {
            return Err("Active boot read-back mismatch".into());
        }
        Ok::<_, String>(())
    })();
    if let Err(e) = activated {
        if old.exists() {
            fs::rename(&old, root.join(format!(".loopclub-failed-{name}"))).map_err(error)?;
        }
        fs::rename(backup.join("autorun-original"), &old).map_err(error)?;
        return Err(e);
    }
    let _ = fs::File::open(root).and_then(|f| f.sync_all());
    Ok(
        serde_json::json!({"persisted":true,"runtime_sha256":expected,"backup":backup,"files_readback_verified":new_hashes.len(),"old_files_preserved":old_hashes.len(),"restart":false}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (PathBuf, PathBuf, String, String) {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("loopclub-persist-{}-{stamp}", std::process::id()));
        let active = root.join("autorun");
        fs::create_dir_all(&active).unwrap();
        let mut old = vec![0; 64];
        old[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        fs::write(active.join("loop-club.so"), &old).unwrap();
        let before = hash(&active.join("loop-club.so")).unwrap();
        old[40] = 1;
        let runtime = root.join("runtime.so");
        fs::write(&runtime, old).unwrap();
        let after = hash(&runtime).unwrap();
        fs::write(active.join("compatibility.json"),serde_json::to_vec(&serde_json::json!({"product":"Loop Club","boot_mode":"direct","library_sha256":before})).unwrap()).unwrap();
        fs::write(
            active.join("autorun.json"),
            br#"{"value0":{"scripts":[{"path":"autorun.sh","askQuestion":false}]}}"#,
        )
        .unwrap();
        fs::write(active.join("autorun.sh"),format!("#!/bin/sh\n[ \"$(sha256sum \"$DIR/loop-club.so\" | cut -d\" \" -f1)\" = \"{before}\" ] || exit 0\n")).unwrap();
        fs::write(root.join("owner-project"), b"owner work").unwrap();
        (root, runtime, before, after)
    }
    #[test]
    fn direct_update_preserves_old_boot_and_owner_data() {
        let (root, runtime, before, after) = fixture();
        let old = hashes(&root.join("autorun")).unwrap();
        let result = publish(&root, &runtime, &before, &after).unwrap();
        assert_eq!(
            hashes(&PathBuf::from(result["backup"].as_str().unwrap()).join("autorun-original"))
                .unwrap(),
            old
        );
        assert_eq!(hash(&root.join("autorun/loop-club.so")).unwrap(), after);
        assert_eq!(fs::read(root.join("owner-project")).unwrap(), b"owner work");
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn stale_identity_never_moves_existing_boot() {
        let (root, runtime, _, after) = fixture();
        let old = hashes(&root.join("autorun")).unwrap();
        assert!(publish(&root, &runtime, &"0".repeat(64), &after).is_err());
        assert_eq!(hashes(&root.join("autorun")).unwrap(), old);
        assert!(!root.join(".LoopClubBackups").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn mixed_boot_folder_refused_without_writes() {
        let (root, runtime, before, after) = fixture();
        fs::create_dir(root.join("autorun/mods")).unwrap();
        assert!(publish(&root, &runtime, &before, &after).is_err());
        assert!(root.join("autorun/mods").is_dir());
        assert!(!root.join(".LoopClubBackups").exists());
        fs::remove_dir_all(root).unwrap();
    }
}
