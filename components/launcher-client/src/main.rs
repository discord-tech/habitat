// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate habitat_launcher_client as launcher_client;
extern crate habitat_launcher_protocol as protocol;
extern crate ipc_channel;
extern crate libc;
extern crate protobuf;

pub mod error;

use std::collections::HashMap;
use std::thread;
use std::time;

use launcher_client::{LauncherCli, Signal};
use libc::pid_t;

fn main() {
    let client = match launcher_client::env_pipe() {
        Ok(pipe) => LauncherCli::connect(pipe).unwrap(),
        _ => panic!("MUST START FROM LAUNCHER"),
    };
    loop {
        start_restart_terminate(&client);
        start_signal(&client);
        start_wait(&client);
    }
}

fn start_restart_terminate(client: &LauncherCli) {
    match client.spawn(
        "start-restart-terminate",
        "/src/run",
        "hab",
        "hab",
        HashMap::default(),
    ) {
        Ok(pid) => {
            println!("Restart: spawned {}", pid);
            match client.restart(pid) {
                Ok(pid) => {
                    println!("Restart: ok, {}", pid);
                    match client.terminate(pid) {
                        Ok(pid) => println!("Terminate: ok, {}", pid),
                        Err(err) => println!("Terminate: err, {}", err),
                    }
                }
                Err(err) => println!("Restart: err, {}", err),
            }
        }
        Err(err) => println!("Restart: error spawning process, {}", err),
    }
}

fn start_signal(client: &LauncherCli) {
    match client.spawn("start-signal", "/src/run", "hab", "hab", HashMap::default()) {
        Ok(pid) => {
            println!("Signal: spawned {}", pid);
            match client.signal(pid, Signal::USR1) {
                Ok(()) => println!("Signal: ok, {}", pid),
                Err(err) => println!("Signal: err, {}", err),
            }
        }
        Err(err) => println!("Signal: error spawning process, {}", err),
    }
}

fn start_wait(client: &LauncherCli) {
    let wait_time = time::Duration::from_millis(1_000);
    let mut pid = 0;
    loop {
        if pid != 0 && pid_alive(pid) {
            thread::sleep(wait_time);
            continue;
        }
        match client.spawn("start-wait", "/src/run", "hab", "hab", HashMap::default()) {
            Ok(new_pid) => {
                pid = new_pid;
                println!("Spawned {}", pid);
            }
            Err(err) => println!("Error spawning process, {}", err),
        }
        thread::sleep(wait_time);
    }
}

fn pid_alive(pid: u32) -> bool {
    match unsafe { libc::kill(pid as pid_t, 0) } {
        0 => true,
        _ => false,
    }
}
