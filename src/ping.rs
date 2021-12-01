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

use super::colors::colorize;
use std::io::{BufRead, Result};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Ping = (f32, String);

fn parse_ping(output: Result<String>) -> Option<Ping> {
    let output = output.ok()?;
    if output.contains("time=") {
        let mut ping_output = output.split("=").last().unwrap().split(" ");
        let ping: f32 = ping_output.next().unwrap().parse().unwrap();
        let unit = ping_output.next().unwrap();
        Some((ping, unit.to_string()))
    } else {
        None
    }
}

pub struct PingManager {
    ping: Arc<Mutex<Option<Ping>>>,
    host: String,
}

impl PingManager {
    pub fn new(host: String) -> PingManager {
        let ping_manager = PingManager {
            host,
            ping: Arc::new(Mutex::new(None)),
        };
        ping_manager.start();
        ping_manager
    }

    fn start(self: &PingManager) -> () {
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

    pub fn wait(self: &PingManager) -> () {
        // Wait (at most 200ms) for the first ping.
        for _ in 0..20 {
            if self.ping.lock().unwrap().is_some() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn current(self: &PingManager) -> String {
        // Get the current ping (or N/A if we couldn't obtain it)
        match self.ping.lock().unwrap().as_ref() {
            None => "N/A".to_string(),
            Some(ping) => colorize(format!("  {} {}", ping.0, ping.1), ping.0, 30.0, 100.0),
        }
    }
}
