// Copyright (c) 2017, MesaLock Linux Authors.
// All rights reserved.
// 
// This work is licensed under the terms of the BSD 3-Clause License.
// For a copy, see the LICENSE file.

extern crate libc;
extern crate nix;
use nix::unistd;

use libc::{waitpid, sigprocmask, sigfillset, sigset_t, signal};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::mem;
use std::ptr;
use std::ffi::CString;

fn run(line: &str) {
    println!("[+] init: run {}", line);
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
                    // TODO: Open the new terminal device
                    Ok(())
                }).spawn() {
                    Ok(mut child) => match child.wait() {
                        Ok(_status) => {
                            println!("[+] init: {} exit", line);
                            unsafe { sigprocmask_allsigs(libc::SIG_UNBLOCK); }
                        },
                        Err(err) => println!("[-] init: failed to wait: {}", err)
                    },
                    Err(err) => println!("[-] init: failed to execute: {}", err)
                }
            }
        }

    }
}

unsafe fn sigprocmask_allsigs(how: libc::c_int) {
    let mut sigset = mem::uninitialized::<sigset_t>();
    sigfillset(&mut sigset as *mut sigset_t);
    sigprocmask(how, &sigset as *const sigset_t, ptr::null_mut() as *mut sigset_t);
}


unsafe fn reset_sighandlers_and_unblock_sigs() {
    signal(libc::SIGUSR1, libc::SIG_DFL);
    signal(libc::SIGUSR2, libc::SIG_DFL);
    signal(libc::SIGTERM, libc::SIG_DFL);
    signal(libc::SIGQUIT, libc::SIG_DFL);
    signal(libc::SIGINT, libc::SIG_DFL);
    signal(libc::SIGHUP, libc::SIG_DFL);
    signal(libc::SIGTSTP, libc::SIG_DFL);
    signal(libc::SIGSTOP, libc::SIG_DFL);
    sigprocmask_allsigs(libc::SIG_UNBLOCK);
}

fn main() {
    println!("init");
    unistd::setsid().expect("setsid failed");
    unsafe {
        libc::putenv(CString::new("HOME=/").unwrap().into_raw());
        libc::putenv(CString::new("PATH=/sbin:/bin:/usr/sbin:/usr/bin").unwrap().into_raw());
        libc::putenv(CString::new("SHELL=/bin/sh").unwrap().into_raw());
    }

    // TODO: setup signal handler

    run("/bin/busybox mount -n -t proc proc /proc");
    run("/bin/busybox mount -n -t devtmpfs devtmpfs /dev");
    run("/bin/busybox mount -n -t sysfs sysfs /sys");

    run("mknod -m 600 /dev/console c 5 1");
    run("mknod -m 620 /dev/tty1 c 4 1");
    run("mknod -m 666 /dev/tty c 5 0");
    run("mknod -m 666 /dev/null c 1 3");
    run("mknod -m 660 /dev/kmsg c 1 11");

    run("/bin/mgetty");
    loop {
        let mut status = 0;
        unsafe {
            waitpid(0, &mut status, 0);
        }
    }
}
