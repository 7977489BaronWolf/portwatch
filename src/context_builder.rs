//! Builds PortContext from live system data (procfs on Linux, stubs elsewhere).

use crate::context::PortContext;

pub struct ContextBuilder {
    use_proc: bool,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self { use_proc: cfg!(target_os = "linux") }
    }

    pub fn with_proc(mut self, enabled: bool) -> Self {
        self.use_proc = enabled;
        self
    }

    /// Build a context for the given port/protocol, optionally resolving
    /// process info from /proc/net on Linux.
    pub fn build(&self, port: u16, protocol: &str) -> PortContext {
        let mut ctx = PortContext::new(port, protocol);

        if self.use_proc {
            if let Some((pid, name, user)) = resolve_proc(port, protocol) {
                ctx = ctx.with_pid(pid).with_process(name).with_user(user);
            }
        }

        ctx
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Attempt to resolve process info for a port from /proc.
/// Returns (pid, process_name, username) or None.
fn resolve_proc(port: u16, protocol: &str) -> Option<(u32, String, String)> {
    let path = match protocol {
        "tcp" => "/proc/net/tcp",
        "tcp6" => "/proc/net/tcp6",
        "udp" => "/proc/net/udp",
        _ => return None,
    };

    let content = std::fs::read_to_string(path).ok()?;
    let hex_port = format!("{:04X}", port);

    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 8 {
            continue;
        }
        // local_address is cols[1]: hex_ip:hex_port
        if let Some(p) = cols[1].split(':').nth(1) {
            if p.eq_ignore_ascii_case(&hex_port) {
                let inode = cols.get(9).copied().unwrap_or("0");
                let pid = find_pid_for_inode(inode).unwrap_or(0);
                let name = read_process_name(pid).unwrap_or_else(|| "unknown".into());
                return Some((pid, name, "unknown".into()));
            }
        }
    }
    None
}

fn find_pid_for_inode(inode: &str) -> Option<u32> {
    let target = format!("socket:[{}]", inode);
    let procs = std::fs::read_dir("/proc").ok()?;
    for entry in procs.flatten() {
        let pid_str = entry.file_name();
        let pid: u32 = pid_str.to_string_lossy().parse().ok()?;
        let fd_dir = format!("/proc/{}/fd", pid);
        if let Ok(fds) = std::fs::read_dir(&fd_dir) {
            for fd in fds.flatten() {
                if let Ok(link) = std::fs::read_link(fd.path()) {
                    if link.to_string_lossy() == target {
                        return Some(pid);
                    }
                }
            }
        }
    }
    None
}

fn read_process_name(pid: u32) -> Option<String> {
    let path = format!("/proc/{}/comm", pid);
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
}
