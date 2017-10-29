extern crate libc;

use libc::waitpid;
use std::process::Command;

fn run(line: &str) {
    let mut args = line.split(' ').map(|arg| {arg.to_string()});

    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            _ => {
                let mut command = Command::new(cmd);
                for arg in args {
                    command.arg(arg);
                }

                match command.spawn() {
                    Ok(mut child) => match child.wait() {
                        Ok(_status) => (),
                        Err(err) => println!("[-] init: failed to wait: {}", err)
                    },
                    Err(err) => println!("[-] init: failed to execute: {}", err)
                }
            }
        }

    }
}

fn main() {
    run("/bin/ion");

    loop {
        let mut status = 0;
        unsafe {
            waitpid(0, &mut status, 0);
        }
    }
}
