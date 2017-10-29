extern crate libc;

use libc::waitpid;
use std::process::Command;

fn main() {
    let cmd = "/bin/ion";
    let mut command = Command::new(cmd);

    match command.spawn() {
        Ok(mut child) => match child.wait() {
            Ok(_status) => (),
                Err(err) => println!("[-] init: failed to wait: {}", err)
        },
        Err(err) => println!("[-] init: failed to execute: {}", err)
    }

    loop {
        let mut status = 0;
        unsafe {
            waitpid(0, &mut status, 0);
        }
    }
}
