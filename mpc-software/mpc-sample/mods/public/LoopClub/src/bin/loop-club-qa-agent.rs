//! Opt-in development agent, isolated from ac50 by a transient systemd unit.
//! Updates only /run/loop-club.so. Never exports SD, writes firmware, runs an
//! uploaded script, or edits owner projects. Device power-off removes it.
use std::{
    fs,
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};
const LIMIT: usize = 8 * 1024 * 1024;
const ROOT: &str = "/run/loop-club-qa-agent";
const LIVE: &str = "/run/loop-club.so";
fn hash(path: &Path) -> Result<String, String> {
    let result = Command::new("sha256sum")
        .arg(path)
        .output()
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err("sha256sum failed".into());
    }
    let sha = String::from_utf8_lossy(&result.stdout)
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_owned();
    if !valid_hash(&sha) {
        return Err("Invalid SHA-256 result".into());
    }
    Ok(sha)
}
fn valid_hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn elf(bytes: &[u8]) -> bool {
    bytes.len() >= 64
        && bytes.len() <= LIMIT
        && &bytes[..4] == b"\x7fELF"
        && bytes[4..7] == [2, 1, 1]
        && bytes[16..20] == [3, 0, 183, 0]
}
fn regular(path: &Path) -> Result<(), String> {
    if fs::symlink_metadata(path)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_file()
    {
        Ok(())
    } else {
        Err("Expected regular file; links refused".into())
    }
}
fn stage(root: &Path, body: &[u8], expected: &str) -> Result<String, String> {
    if !valid_hash(expected) || !elf(body) {
        return Err("Expected ARM64 ELF shared library and lowercase SHA-256".into());
    }
    let incoming = root.join("incoming.so");
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&incoming)
        .map_err(|e| e.to_string())?;
    let result = (|| {
        f.write_all(body).map_err(|e| e.to_string())?;
        f.sync_all().map_err(|e| e.to_string())?;
        let actual = hash(&incoming)?;
        if actual != expected {
            return Err("Upload SHA-256 mismatch".into());
        }
        fs::rename(&incoming, root.join("staged.so")).map_err(|e| e.to_string())?;
        Ok(actual)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&incoming);
    }
    result
}
fn replace(source: &Path, live: &Path) -> Result<(), String> {
    regular(source)?;
    regular(live)?;
    let tmp = live.with_extension("qa-new");
    let mut out = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp)
        .map_err(|e| e.to_string())?;
    let result = (|| {
        let mut input = fs::File::open(source).map_err(|e| e.to_string())?;
        std::io::copy(&mut input, &mut out).map_err(|e| e.to_string())?;
        out.sync_all().map_err(|e| e.to_string())?;
        if hash(&tmp)? != hash(source)? {
            return Err("Runtime read-back mismatch".into());
        }
        fs::rename(&tmp, live).map_err(|e| e.to_string())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}
fn app_status() -> Option<serde_json::Value> {
    let mut stream = TcpStream::connect_timeout(
        &"192.168.155.1:8090".parse().ok()?,
        Duration::from_millis(500),
    )
    .ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(500)))
        .ok()?;
    stream
        .write_all(b"GET /status.json HTTP/1.1\r\nHost: 192.168.155.1\r\nConnection: close\r\n\r\n")
        .ok()?;
    let mut bytes = Vec::new();
    stream.take(65536).read_to_end(&mut bytes).ok()?;
    let start = bytes.windows(4).position(|w| w == b"\r\n\r\n")? + 4;
    serde_json::from_slice(&bytes[start..]).ok()
}
fn restart() -> Result<(), String> {
    if Command::new("systemctl")
        .args(["restart", "ac50.service"])
        .status()
        .map_err(|e| e.to_string())?
        .success()
    {
        Ok(())
    } else {
        Err("ac50 restart failed".into())
    }
}
fn rollback(
    live: &Path,
    previous: &Path,
    pending: &mut Option<String>,
    do_restart: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    replace(previous, live)?;
    // Clear the failed candidate even when the recovery restart itself fails.
    *pending = None;
    do_restart()
}
fn boot(root: &Path, pending: &mut Option<String>) -> Result<serde_json::Value, String> {
    let before = app_status();
    if let Some(s) = &before {
        let words = s["words"].as_array().ok_or("Missing live snapshot")?;
        if words.len() < 10
            || words[4].as_u64().unwrap_or(1) & 1 != 0
            || words[9].as_u64().unwrap_or(1) != 0
        {
            return Err("Stop playback and recording before restart".into());
        }
    }
    let old_pid = before.as_ref().and_then(|v| v["pid"].as_u64());
    let live = Path::new(LIVE);
    let previous = root.join("previous.so");
    let mut changed = false;
    if let Some(expected) = pending.as_ref() {
        let staged = root.join("staged.so");
        if hash(&staged)? != *expected {
            return Err("Staged library changed".into());
        }
        regular(live)?;
        // Preserve the currently working library before publishing a new inode.
        fs::copy(live, &previous).map_err(|e| e.to_string())?;
        if hash(&previous)? != hash(live)? {
            return Err("Backup read-back mismatch".into());
        }
        replace(&staged, live)?;
        changed = true;
    }
    let result = (|| {
        restart()?;
        let started = Instant::now();
        while started.elapsed() < Duration::from_secs(20) {
            if let Some(s) = app_status() {
                let pid = s["pid"].as_u64();
                if pid.is_some() && pid != old_pid {
                    return Ok(
                        serde_json::json!({"result":"restarted","app_pid":pid,"build_id":s["build_id"],"runtime_sha256":hash(live)?}),
                    );
                }
            }
            std::thread::sleep(Duration::from_millis(300));
        }
        Err("New QA application did not become reachable".into())
    })();
    if result.is_err() && changed {
        rollback(live, &previous, pending, restart)?;
        return Err(
            "Candidate failed health check; previous runtime library restored and app restarted"
                .into(),
        );
    }
    if result.is_ok() {
        *pending = None;
    }
    result
}
fn reply(s: &mut TcpStream, status: &str, value: serde_json::Value) {
    let body = serde_json::to_vec(&value).unwrap_or_default();
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = s
        .write_all(header.as_bytes())
        .and_then(|_| s.write_all(&body));
}
fn serve(mut s: TcpStream, root: &Path, pending: &mut Option<String>) {
    if !s.peer_addr().is_ok_and(|a|matches!(a.ip(),std::net::IpAddr::V4(ip) if ip.octets()[..3]==[192,168,155] && ip.octets()[3]<16)) { return; }
    let _ = s.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = s.set_write_timeout(Some(Duration::from_secs(5)));
    let mut raw = Vec::new();
    let mut block = [0; 8192];
    let end = loop {
        if raw.len() > 16384 {
            return;
        }
        let Ok(n) = s.read(&mut block) else { return };
        if n == 0 {
            return;
        };
        raw.extend_from_slice(&block[..n]);
        if let Some(i) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
            break i + 4;
        }
    };
    let header = String::from_utf8_lossy(&raw[..end]).to_lowercase();
    let first = header.lines().next().unwrap_or("");
    if first.starts_with("get /status.json ") {
        let last = fs::read(root.join("last-result.json"))
            .ok()
            .and_then(|b| serde_json::from_slice::<serde_json::Value>(&b).ok());
        reply(
            &mut s,
            "200 OK",
            serde_json::json!({"agent":"loop-club-qa-agent-v1","scope":"volatile /run only; no SD or firmware changes","pending_sha256":pending,"last_result":last,"app":app_status().map(|v|serde_json::json!({"pid":v["pid"],"build_id":v["build_id"]}))}),
        );
        return;
    }
    if header.contains("\r\norigin:") || !header.lines().any(|h| h == "x-loop-club-management: qa4")
    {
        reply(
            &mut s,
            "403 Forbidden",
            serde_json::json!({"error":"USB QA management header required; browser origins refused"}),
        );
        return;
    }
    let len = header
        .lines()
        .find_map(|h| {
            h.strip_prefix("content-length:")?
                .trim()
                .parse::<usize>()
                .ok()
        })
        .unwrap_or(0);
    let cap = if first.starts_with("post /stage ") {
        LIMIT
    } else {
        128
    };
    if len == 0 || len > cap {
        reply(
            &mut s,
            "413 Payload Too Large",
            serde_json::json!({"error":"Invalid upload length"}),
        );
        return;
    }
    while raw.len() < end + len {
        let Ok(n) = s.read(&mut block) else { return };
        if n == 0 {
            return;
        };
        raw.extend_from_slice(&block[..n]);
        if raw.len() > end + cap {
            return;
        }
    }
    let body = &raw[end..end + len];
    if first.starts_with("post /stage ") {
        let sha = header
            .lines()
            .find_map(|h| h.strip_prefix("x-content-sha256:"))
            .map(str::trim)
            .unwrap_or("");
        match stage(root, body, sha) {
            Ok(sha) => {
                *pending = Some(sha.clone());
                reply(&mut s, "200 OK", serde_json::json!({"staged_sha256":sha}));
            }
            Err(e) => reply(&mut s, "400 Bad Request", serde_json::json!({"error":e})),
        };
        return;
    }
    if first.starts_with("post /restart ")
        && serde_json::from_slice::<serde_json::Value>(body)
            .is_ok_and(|v| v == serde_json::json!({"discard_session":true}))
    {
        // Client explicitly authorizes discarding this QA app session. A separate
        // systemd unit keeps this agent alive while ac50 is restarted.
        reply(
            &mut s,
            "202 Accepted",
            serde_json::json!({"restart":"queued","poll":"/status.json"}),
        );
        let result = boot(root, pending).unwrap_or_else(|e| serde_json::json!({"error":e}));
        let _ = fs::write(
            root.join("last-result.json"),
            serde_json::to_vec_pretty(&result).unwrap_or_default(),
        );
        return;
    }
    reply(
        &mut s,
        "404 Not Found",
        serde_json::json!({"error":"Unknown QA operation"}),
    );
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() != Some("--usb-only") {
        return Err("Only the explicit --usb-only development launcher is supported".into());
    }
    let root = PathBuf::from(ROOT);
    fs::create_dir_all(&root)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
    }
    let mut listener = None;
    for _ in 0..120 {
        match TcpListener::bind((Ipv4Addr::new(192, 168, 155, 1), 8091)) {
            Ok(l) => {
                listener = Some(l);
                break;
            }
            Err(_) => std::thread::sleep(Duration::from_secs(1)),
        }
    }
    let listener = listener.ok_or("USB agent bind failed")?;
    let mut pending = None;
    for s in listener.incoming().flatten() {
        serve(s, &root, &mut pending);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reject_wrong_elf_architecture_and_hash() {
        let mut b = vec![0; 64];
        b[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        b[16..20].copy_from_slice(&[3, 0, 183, 0]);
        assert!(elf(&b));
        b[18] = 62;
        assert!(!elf(&b));
        assert!(!elf(&[]));
        assert!(!valid_hash("../bad"));
        assert!(!valid_hash(&"A".repeat(64)));
        assert!(valid_hash(&"0".repeat(64)));
    }
    #[test]
    fn stage_checks_hash_and_atomic_replace_keeps_old_inode() {
        let root = std::env::temp_dir().join(format!("loopclub-agent-test-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let mut b = vec![0; 64];
        b[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        b[16..20].copy_from_slice(&[3, 0, 183, 0]);
        let sample = root.join("sample.so");
        fs::write(&sample, &b).unwrap();
        let sha = hash(&sample).unwrap();
        assert!(stage(&root, &b, &"0".repeat(64)).is_err());
        assert!(!root.join("incoming.so").exists());
        assert_eq!(stage(&root, &b, &sha).unwrap(), sha);
        let live = root.join("live.so");
        fs::write(&live, b"old library").unwrap();
        let mut open = fs::File::open(&live).unwrap();
        replace(&root.join("staged.so"), &live).unwrap();
        let mut old = Vec::new();
        open.read_to_end(&mut old).unwrap();
        assert_eq!(old, b"old library");
        assert_eq!(fs::read(&live).unwrap(), b);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&sample, root.join("link.so")).unwrap();
            assert!(replace(&root.join("link.so"), &live).is_err());
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn failed_candidate_restores_previous_before_restart_and_clears_pending() {
        let root =
            std::env::temp_dir().join(format!("loopclub-rollback-test-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let live = root.join("live.so");
        let previous = root.join("previous.so");
        fs::write(&live, b"failed candidate").unwrap();
        fs::write(&previous, b"known working").unwrap();
        let mut pending = Some("failed hash".into());
        let result = rollback(&live, &previous, &mut pending, || {
            assert_eq!(fs::read(&live).unwrap(), b"known working");
            Err("injected recovery restart failure".into())
        });
        assert!(result.is_err());
        assert!(pending.is_none());
        assert_eq!(fs::read(&live).unwrap(), b"known working");
        fs::remove_dir_all(root).unwrap();
    }
}
