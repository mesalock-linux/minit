extern crate libc;

use libc::{waitpid, sigprocmask, sigfillset, sigset_t, signal};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::mem;
use std::ptr;

fn run(line: &str) {
    let mut args = line.split(' ').map(|arg| {arg.to_string()});

    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            _ => {
                let mut command = Command::new(cmd);
                for arg in args {
                    command.arg(arg);
                }

                match command.before_exec(|| {
                    unsafe { reset_sighandlers_and_unblock_sigs() }
                    Ok(())
                }).spawn() {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(_status) => (),
                            Err(err) => println!("[-] init: failed to wait: {}", err)
                        }
                    },
                    Err(err) => println!("[-] init: failed to execute: {}", err)
                }
            }
        }

    }
}

unsafe fn unblock_signals() {
    let mut sigset = mem::uninitialized::<sigset_t>();
    sigfillset(&mut sigset as *mut sigset_t);
    sigprocmask(libc::SIG_UNBLOCK, &sigset as *const sigset_t, ptr::null_mut() as *mut sigset_t);
}

unsafe fn block_signals() {
    let mut sigset = mem::uninitialized::<sigset_t>();
    sigfillset(&mut sigset as *mut sigset_t);
    sigprocmask(libc::SIG_BLOCK, &sigset as *const sigset_t, ptr::null_mut() as *mut sigset_t);
}

unsafe fn reset_sighandlers_and_unblock_sigs() {
    /*
     * bb_signals(0
		+ (1 << SIGUSR1)
		+ (1 << SIGUSR2)
		+ (1 << SIGTERM)
		+ (1 << SIGQUIT)
		+ (1 << SIGINT)
		+ (1 << SIGHUP)
		+ (1 << SIGTSTP)
		+ (1 << SIGSTOP)
		, SIG_DFL);
        */
    println!("reset_sighandlers_and_unblock_sigs");
    signal(libc::SIGUSR1, libc::SIG_DFL);
    signal(libc::SIGUSR2, libc::SIG_DFL);
    signal(libc::SIGTERM, libc::SIG_DFL);
    signal(libc::SIGQUIT, libc::SIG_DFL);
    signal(libc::SIGINT, libc::SIG_DFL);
    signal(libc::SIGHUP, libc::SIG_DFL);
    signal(libc::SIGTSTP, libc::SIG_DFL);
    signal(libc::SIGSTOP, libc::SIG_DFL);
    unblock_signals();
}

fn main() {
    unsafe { block_signals() }
    unsafe { reset_sighandlers_and_unblock_sigs() }
    run("/bin/busybox sh");

    loop {
        let mut status = 0;
        unsafe {
            waitpid(0, &mut status, 0);
        }
    }
}
