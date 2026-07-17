/*
 * File: utils.rs
 * Project: docker-native-manager
 * Created: 2026-03-17
 * 
 * Last Modified: Wed Apr 01 2026
 * Modified By: Pedro Farias
 * 
 */

use bollard::Docker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::mpsc;
use std::process::Child;
use std::io::Read;
use crate::docker_context;

pub static IS_STOPPED_INTENTIONALLY: AtomicBool = AtomicBool::new(false);
pub static WSL_DOCKER_DETECTED: AtomicBool = AtomicBool::new(false);

pub type TerminalSenders = Mutex<HashMap<String, mpsc::Sender<String>>>;

// Global SSH tunnel process handle
lazy_static::lazy_static! {
    static ref SSH_TUNNEL: Mutex<Option<SshTunnel>> = Mutex::new(None);
    static ref WSL_DOCKER: Mutex<Option<WslForwarder>> = Mutex::new(None);
}

struct SshTunnel {
    child: Child,
    local_addr: String,
}

impl Drop for SshTunnel {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        #[cfg(unix)]
        let _ = std::fs::remove_file(&self.local_addr);
    }
}

/// Stop any existing SSH tunnel
pub fn stop_ssh_tunnel() {
    let mut tunnel = SSH_TUNNEL.lock().unwrap();
    if let Some(mut t) = tunnel.take() {
        let _ = t.child.kill();
        let _ = t.child.wait();
        #[cfg(unix)]
        let _ = std::fs::remove_file(&t.local_addr);
        // Prevent Drop from running again
        std::mem::forget(t);
    }
}

#[cfg(windows)]
fn run_wsl(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    let make = |exe: &str| {
        let mut c = Command::new(exe);
        c.args(args)
         .stdout(std::process::Stdio::null())
         .stderr(std::process::Stdio::null())
         .creation_flags(0x08000000);
        c
    };
    make("wsl.exe").output().or_else(|_| make("C:\\Windows\\System32\\wsl.exe").output())
}

#[cfg(windows)]
fn try_wsl_docker() -> bool {
    if WSL_DOCKER_DETECTED.load(Ordering::Relaxed) {
        return true;
    }
    let ok = run_wsl(&["docker", "version", "--format", "{{.Server.Version}}"])
        .map_or(false, |o| o.status.success())
        ||
        run_wsl(&["python3", "-c",
            "import socket;s=socket.socket(socket.AF_UNIX);s.settimeout(2);s.connect('/var/run/docker.sock');s.close()"
        ]).map_or(false, |o| o.status.success());
    if ok {
        WSL_DOCKER_DETECTED.store(true, Ordering::Relaxed);
    }
    ok
}

#[cfg(windows)]
fn write_wsl_script(port: u16) -> Result<String, String> {
    let script = format!(
r#"import socket, threading, sys, os

def forward(a, b):
    try:
        while True:
            d = a.recv(4096)
            if not d:
                break
            b.sendall(d)
    except:
        pass
    finally:
        try: a.close()
        except: pass
        try: b.close()
        except: pass

s = socket.socket()
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(('127.0.0.1', {}))
s.listen(5)
sys.stdout.write('R')
sys.stdout.flush()
while True:
    c, _ = s.accept()
    d = socket.socket(socket.AF_UNIX)
    d.connect('/var/run/docker.sock')
    threading.Thread(target=forward, args=(c, d), daemon=True).start()
    threading.Thread(target=forward, args=(d, c), daemon=True).start()
"#, port);

    let script_path = std::env::temp_dir().join("docker_nm_wsl_proxy.py");
    std::fs::write(&script_path, &script)
        .map_err(|e| format!("Failed to write WSL proxy script: {}", e))?;

    // Convert Windows path to WSL path: C:\Users\... → /mnt/c/Users/...
    let wsl_path = script_path.to_string_lossy()
        .replace('\\', "/")
        .replacen("C:", "/mnt/c", 1)
        .replacen("D:", "/mnt/d", 1)
        .replacen("E:", "/mnt/e", 1);
    Ok(wsl_path)
}

#[cfg(windows)]
fn start_wsl_forwarder(port: u16) -> Result<Child, String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;

    let wsl_path = write_wsl_script(port)?;

    let mut child = Command::new("wsl.exe")
        .args(["python3", &wsl_path])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .creation_flags(0x08000000)
        .spawn()
        .or_else(|_| {
            Command::new("C:\\Windows\\System32\\wsl.exe")
                .args(["python3", &wsl_path])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .creation_flags(0x08000000)
                .spawn()
        })
        .map_err(|e| format!("Failed to start WSL Docker forwarder: {}", e))?;

    let mut stdout = child.stdout.take().ok_or("No stdout from WSL forwarder")?;
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);
    let mut buf = [0u8; 1];

    loop {
        match stdout.read_exact(&mut buf) {
            Ok(()) if buf[0] == b'R' => break,
            Ok(()) => {}
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("WSL forwarder failed to send ready signal.".to_string());
            }
        }

        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err("WSL Docker forwarder timed out.".to_string());
        }

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(child)
}

struct WslForwarder {
    child: Child,
    addr: String,
}

impl Drop for WslForwarder {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn stop_wsl_forwarder() {
    let mut fwd = WSL_DOCKER.lock().unwrap();
    if let Some(mut f) = fwd.take() {
        let _ = f.child.kill();
        let _ = f.child.wait();
        std::mem::forget(f);
    }
}

#[cfg(windows)]
fn find_available_port() -> Result<u16, String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to find available port: {}", e))?;
    let port = listener.local_addr()
        .map_err(|e| format!("Failed to get port: {}", e))?
        .port();
    drop(listener);
    Ok(port)
}

/// Internal helper to start an SSH tunnel without locking the global mutex
fn start_ssh_tunnel_raw(ssh_url: &str) -> Result<(Child, String), String> {
    let url_part = ssh_url.trim_start_matches("ssh://");

    let local_addr = build_local_addr();
    let forward_spec = format!("{}:/var/run/docker.sock", local_addr);

    // Build SSH command: forward remote Docker socket to local address
    let mut cmd = std::process::Command::new("ssh");
    cmd.arg("-N")
       .arg("-o").arg("StrictHostKeyChecking=accept-new")
       .arg("-o").arg("ConnectTimeout=10")
       .arg("-o").arg("ServerAliveInterval=30")
       .arg("-o").arg("ServerAliveCountMax=3")
       .arg("-o").arg("ExitOnForwardFailure=yes")
       .arg("-L").arg(&forward_spec);

    // Parse port if present (user@host:port)
    if let Some(at_pos) = url_part.find('@') {
        let user = &url_part[..at_pos];
        let host_part = &url_part[at_pos + 1..];

        if let Some(colon_pos) = host_part.rfind(':') {
            let host = &host_part[..colon_pos];
            let port = &host_part[colon_pos + 1..];
            cmd.arg("-p").arg(port);
            cmd.arg(format!("{}@{}", user, host));
        } else {
            cmd.arg(format!("{}@{}", user, host_part));
        }
    } else {
        if let Some(colon_pos) = url_part.rfind(':') {
            let host = &url_part[..colon_pos];
            let port = &url_part[colon_pos + 1..];
            cmd.arg("-p").arg(port);
            cmd.arg(host);
        } else {
            cmd.arg(url_part);
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let mut child = cmd
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start SSH tunnel: {}", e))?;

    // Wait for the tunnel to be ready
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(15);

    loop {
        if is_tunnel_ready(&local_addr) {
            std::thread::sleep(std::time::Duration::from_millis(500));
            break;
        }

        match child.try_wait() {
            Ok(Some(status)) => {
                let mut err_msg = String::new();
                if let Some(mut stderr) = child.stderr.take() {
                    let _ = stderr.read_to_string(&mut err_msg);
                }
                cleanup_addr(&local_addr);

                if err_msg.contains("Permission denied") || err_msg.contains("publickey") {
                    return Err(format!("SSH authentication failed. Check your SSH key configuration.\n\nDetails: {}", err_msg.trim()));
                }
                if err_msg.contains("Connection refused") || err_msg.contains("Connection timed out") {
                    return Err(format!("Cannot reach remote host. Check hostname/IP and SSH port.\n\nDetails: {}", err_msg.trim()));
                }
                if !err_msg.trim().is_empty() {
                    return Err(format!("SSH tunnel exited (code {}): {}", status, err_msg.trim()));
                }
                return Err(format!("SSH tunnel exited with code {}. Check SSH connectivity.", status));
            },
            Ok(None) => {},
            Err(e) => {
                cleanup_addr(&local_addr);
                return Err(format!("Failed to check SSH tunnel status: {}", e));
            }
        }

        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            cleanup_addr(&local_addr);
            return Err("SSH tunnel timed out.".to_string());
        }

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok((child, local_addr))
}

#[cfg(unix)]
fn build_local_addr() -> String {
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() % 100000;
    let mut temp = std::env::temp_dir();
    temp.push(format!("docker-nm-ssh-{}-{}.sock", std::process::id(), unique_id));
    let path = temp.to_string_lossy().to_string();
    if std::path::Path::new(&path).exists() {
        let _ = std::fs::remove_file(&path);
    }
    path
}

#[cfg(windows)]
fn build_local_addr() -> String {
    let port = find_available_port().unwrap_or(23456);
    format!("127.0.0.1:{}", port)
}

#[cfg(unix)]
fn is_tunnel_ready(addr: &str) -> bool {
    std::path::Path::new(addr).exists()
}

#[cfg(windows)]
fn is_tunnel_ready(addr: &str) -> bool {
    std::net::TcpStream::connect(addr).is_ok()
}

#[cfg(unix)]
fn cleanup_addr(addr: &str) {
    let _ = std::fs::remove_file(addr);
}

#[cfg(windows)]
fn cleanup_addr(_addr: &str) {
    // Windows uses TCP ports, nothing to clean up
}

#[cfg(unix)]
fn connect_via_addr(addr: &str) -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_socket(addr, 120, bollard::API_DEFAULT_VERSION)
}

#[cfg(windows)]
fn connect_via_addr(addr: &str) -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_http(addr, 120, bollard::API_DEFAULT_VERSION)
}

#[cfg(windows)]
fn try_connect_wsl() -> Option<Result<Docker, String>> {
    if !try_wsl_docker() {
        return None;
    }

    let mut fwd = WSL_DOCKER.lock().ok()?;

    // Test existing forwarder
    if let Some(ref f) = *fwd {
        if std::net::TcpStream::connect(&f.addr).is_ok() {
            return Some(Docker::connect_with_http(&f.addr, 120, bollard::API_DEFAULT_VERSION)
                .map_err(|e| e.to_string()));
        }
    }

    // Kill existing forwarder if broken
    if let Some(mut f) = fwd.take() {
        let _ = f.child.kill();
        let _ = f.child.wait();
        std::mem::forget(f);
    }
    drop(fwd);

    // Start new forwarder
    let port = find_available_port().ok()?;
    let child = start_wsl_forwarder(port).ok()?;
    let addr = format!("127.0.0.1:{}", port);

    *WSL_DOCKER.lock().unwrap() = Some(WslForwarder {
        child,
        addr: addr.clone(),
    });

    // Wait for TCP to be ready
    let start = std::time::Instant::now();
    loop {
        if std::net::TcpStream::connect(&addr).is_ok() {
            break;
        }
        if start.elapsed() > std::time::Duration::from_secs(5) {
            return None;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Some(Docker::connect_with_http(&addr, 120, bollard::API_DEFAULT_VERSION)
        .map_err(|e| e.to_string()))
}

#[cfg(windows)]
fn connect_with_fallback() -> Result<Docker, String> {
    if let Some(result) = try_connect_wsl() {
        return result;
    }
    Docker::connect_with_local_defaults().map_err(|e| e.to_string())
}

#[cfg(not(windows))]
fn connect_with_fallback() -> Result<Docker, String> {
    Docker::connect_with_local_defaults().map_err(|e| e.to_string())
}

pub fn get_docker() -> Result<Docker, String> {
    if IS_STOPPED_INTENTIONALLY.load(Ordering::SeqCst) {
        return Err("Docker is intentionally stopped".into());
    }

    // Get the current context's endpoint
    let host = match std::process::Command::new("docker")
        .args(["context", "inspect", "--format", "{{.Endpoints.docker.Host}}"])
        .output()
    {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            docker_context::get_current_context_endpoint().unwrap_or_default()
        }
        _ => String::new(),
    };

    if host.is_empty() {
        stop_ssh_tunnel();
        return connect_with_fallback();
    }

    if host.starts_with("unix://") || host.starts_with("/") {
        stop_ssh_tunnel();
        connect_with_fallback()
    } else if host.starts_with("ssh://") {
        let mut tunnel_lock = SSH_TUNNEL.lock().unwrap();

        // Check if existing tunnel is valid
        if let Some(ref mut tunnel) = *tunnel_lock {
            let process_alive = tunnel.child.try_wait().map(|s| s.is_none()).unwrap_or(false);
            let addr_ok = is_tunnel_ready(&tunnel.local_addr);

            if addr_ok && process_alive {
                return connect_via_addr(&tunnel.local_addr)
                    .map_err(|e| e.to_string());
            }
        }

        // Restart tunnel atomically
        if let Some(mut t) = tunnel_lock.take() {
            let _ = t.child.kill();
            let _ = t.child.wait();
            cleanup_addr(&t.local_addr);
            std::mem::forget(t);
        }

        let (child, local_addr) = start_ssh_tunnel_raw(&host)?;
        let addr_to_return = local_addr.clone();

        *tunnel_lock = Some(SshTunnel {
            child,
            local_addr,
        });

        connect_via_addr(&addr_to_return)
            .map_err(|e| e.to_string())
    } else if host.starts_with("npipe://") {
        stop_ssh_tunnel();
        #[cfg(windows)]
        {
            Docker::connect_with_named_pipe(&host, 120, bollard::API_DEFAULT_VERSION)
                .map_err(|e| e.to_string())
        }
        #[cfg(not(windows))]
        {
            connect_with_fallback()
        }
    } else if host.starts_with("tcp://") {
        stop_ssh_tunnel();
        let addr = host.trim_start_matches("tcp://");
        Docker::connect_with_http(addr, 120, bollard::API_DEFAULT_VERSION).map_err(|e| e.to_string())
    } else {
        stop_ssh_tunnel();
        connect_with_fallback()
    }
}
