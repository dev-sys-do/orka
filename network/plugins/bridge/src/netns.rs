use cni_plugin::error::CniError;
use futures::Future;
use nix::{
    fcntl::{open, OFlag},
    sched::CloneFlags,
    sys::stat::Mode,
};
use std::{path::PathBuf, process};

fn get_fd_from_path(path: PathBuf) -> Result<i32, CniError> {
    let fd: i32 = open(path.as_path(), OFlag::O_RDONLY, Mode::empty()).map_err(|e| {
        CniError::Generic(format!(
            "Failed to convert {:?} from PathBuf to i32 (fd). (fn get_fd_from_path) {}",
            path, e
        ))
    })?;

    Ok(fd)
}

fn get_current_thread_netns_path() -> PathBuf {
    let pid: u32 = process::id();
    let tid: u64 = unsafe { libc::syscall(libc::SYS_gettid) as u64 };

    // /proc/self/ns/net returns the namespace of the main thread, not
    // of whatever thread it is running on.  Make sure we use the
    // thread's net namespace since the thread is switching around
    PathBuf::from(format!("/proc/{}/task/{}/ns/net", pid, tid))
}

pub async fn exec<F, Fut, E>(netns: PathBuf, f: F) -> Result<E, CniError>
where
    F: FnOnce(i32) -> Fut,
    Fut: Future<Output = Result<E, CniError>>,
{
    let thread_netns: PathBuf = get_current_thread_netns_path();
    let thread_netns_fd: i32 = get_fd_from_path(thread_netns)?;
    let netns_fd: i32 = get_fd_from_path(netns.clone())?;
    let setns_flags: CloneFlags = CloneFlags::empty();

    // WARNING ! [Change namespace from host to container]
    if let Err(e) = nix::sched::setns(netns_fd, setns_flags) {
        return Err(CniError::Generic(format!(
            "Failed to move namespace from host to {:?}. {}",
            netns, e
        )));
    }

    let res: E = f(thread_netns_fd).await?;

    // Switch back ! [Change namespace from container to host]
    if let Err(e) = nix::sched::setns(thread_netns_fd, setns_flags) {
        return Err(CniError::Generic(format!(
            "Failed to move namespace from {:?} to host? {}",
            netns, e
        )));
    }

    Ok(res)
}
