use crate::{consts, prelude::*};
use libc::{close, ioctl, open, O_NOCTTY};
use std::ffi::CString;

// open the requested file
pub fn get_fd(file: &str) -> Result<i32> {
    let file_path = CString::new(file).unwrap();

    match unsafe { open(file_path.as_ptr(), O_NOCTTY) } {
        -1 => Err(Error::Io(std::io::Error::last_os_error())),
        fd => Ok(fd),
    }
}

// close the file of the given file descriptor
pub fn release_fd(fd: i32) -> Result<()> {
    match unsafe { close(fd) } {
        -1 => Err(Error::Io(std::io::Error::last_os_error())),
        _ => Ok(()),
    }
}

// get the current state of the caps lock led
pub fn get_state(fd: i32, state: &mut i32) -> Result<()> {
    match unsafe { ioctl(fd, consts::KDGKBLED, state) } {
        // if an error occurs, close the file descriptor and return the error
        -1 => {
            unsafe { close(fd) };

            Err(Error::Io(std::io::Error::last_os_error()))
        }
        _ => Ok(()),
    }
}

// change the state of the caps lock led
pub fn change_state(fd: i32, state: &mut i32, target_state: i32) -> Result<()> {
    // binary XOR the current state with the target state
    *state ^= target_state;
    match unsafe { ioctl(fd, consts::KDSKBLED, *state) } {
        // if an error occurs, close the file descriptor and return the error
        -1 => {
            unsafe { close(fd) };

            Err(Error::Io(std::io::Error::last_os_error()))
        }
        _ => Ok(()),
    }
}
