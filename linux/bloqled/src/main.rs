mod consts;
mod error;
mod io;
mod prelude;

use io::{change_state, get_fd, get_state, release_fd};
use prelude::Result;

const DELAY: u64 = 500;
const N: u64 = 5;

// wikipedia ioctl example written in rust
fn main() -> Result<()> {
    // get the file descriptor for the tty1 device
    let fd = get_fd("/dev/tty1")?;
    let mut state = 0;
    get_state(fd, &mut state)?;

    // toggle the caps lock led N times every DELAY milliseconds
    (0..N).for_each(|_| {
        // turn on the led
        change_state(fd, &mut state, consts::K_CAPSLOCK).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(DELAY));

        // turn off the led
        change_state(fd, &mut state, consts::K_CAPSLOCK).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(DELAY));
    });

    // clean up the caps lock state
    change_state(fd, &mut state, consts::K_CAPSLOCK)?;
    // close the file descriptor
    release_fd(fd)?;

    Ok(())
}
