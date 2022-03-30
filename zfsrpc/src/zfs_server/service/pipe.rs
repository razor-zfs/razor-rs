use std::io;
use std::os::unix::io::RawFd;

const DEFAULT_PIPE_MAX: usize = 1048576; // from fs/pipe.c

pub(crate) fn max_pipe_size(fd: RawFd) -> io::Result<usize> {
    let current = f_get_pipe_size(fd)?;
    tracing::debug!(current = current, "Current PIPE buffer size");
    if current < DEFAULT_PIPE_MAX {
        let size = f_set_pipe_size(fd, DEFAULT_PIPE_MAX)?;
        tracing::debug!(size = size, "Set PIPE buffer size");
        if size != DEFAULT_PIPE_MAX {
            return f_get_pipe_size(fd);
        }
    }
    Ok(DEFAULT_PIPE_MAX) // fs/pipe.c default
}

fn f_get_pipe_size(fd: RawFd) -> io::Result<usize> {
    let rc = unsafe { libc::fcntl(fd, libc::F_GETPIPE_SZ) };
    if rc < 0 {
        let err = io::Error::last_os_error();
        tracing::error!("fcntl(F_GETPIPE_SZ) failed: {err}");
        Err(err)
    } else {
        Ok(rc as usize)
    }
}

fn f_set_pipe_size(fd: RawFd, size: usize) -> io::Result<usize> {
    let rc = unsafe { libc::fcntl(fd, libc::F_SETPIPE_SZ, size as u32) };
    if rc < 0 {
        let err = io::Error::last_os_error();
        tracing::error!("fcntl(F_SETPIPE_SZ) failed: {err}");
        Err(err)
    } else {
        Ok(rc as usize)
    }
}
