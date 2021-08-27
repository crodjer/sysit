/*
sysit | Check on the system with a quick glance!
Copyright (C) 2021 Rohan Jain

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::io::{BufRead, Result};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn parse_ping(output: Result<String>) -> Option<String> {
    let output = output.ok()?;
    if output.contains("time=") {
        Some(String::from(output.split("=").last().unwrap()))
    } else {
        None
    }
}

pub struct Ping {
    ping: Arc<Mutex<Option<String>>>,
    host: String,
}

impl Ping {
    pub fn new(host: String) -> Ping {
        let ping = Ping {
            host,
            ping: Arc::new(Mutex::new(None)),
        };
        ping.start();
        ping
    }

    fn start(self: &Ping) -> () {
        // Start the ping subprocess and its monitoring thread.
        let ping = self.ping.clone();
        let host = self.host.clone();

        let _child = thread::spawn(move || {
            let mut child = Command::new("ping")
                .arg("-i 1")
                .arg(host)
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to execute child!");

            let child_stdout = child.stdout.as_mut().unwrap();

            let reader = std::io::BufReader::new(child_stdout);
            for line in reader.lines() {
                *ping.lock().unwrap() = parse_ping(line);
            }
        });
    }

    pub fn wait(self: &Ping) -> () {
        // Wait (at most 200ms) for the first ping.
        for _ in 0..20 {
            if self.ping.lock().unwrap().is_some() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn current(self: &Ping) -> String {
        // Get the current ping (or N/A if we couldn't obtain it)
        match self.ping.lock().unwrap().as_ref() {
            None => "N/A",
            Some(ping) => ping.as_str(),
        }
        .to_string()
    }
}
