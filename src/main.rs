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

use clap::Parser;
use std::io::stdout;
use std::io::Result;
use std::io::Write;
use std::thread;
use std::time::Duration;

use sysinfo::{System, SystemExt};
use sysit::config::Config;
use sysit::cpu;
use sysit::memory;
use sysit::ping;
use sysit::sensors;

use colored::control::set_override;

const NEW_LINE: char = '\n';
const CARRIAGE_RETURN: char = '\r';

struct State {
    config: Config,
    system: System,
    ping: ping::Ping,
}

fn line(state: &State) -> String {
    [
        memory::usage(&state.config, &state.system),
        cpu::overview(&state.config, &state.system),
        sensors::temperature(&state.config, &state.system),
        state.ping.current(),
    ]
    .join(" ")
}

fn render_line(delimiter: char, state: &mut State) -> () {
    state.system.refresh_all();
    print!("{}{}", line(state), delimiter);
    ()
}

fn main() -> Result<()> {
    let config: Config = Config::parse();
    let system = System::new_all();
    let ping = ping::Ping::new(config.ping_host.clone());
    ping.wait();

    if config.colors {
        set_override(true);
    } else if config.no_colors {
        set_override(false);
    }

    let mut state = State {
        config,
        system,
        ping,
    };

    let delimiter = if state.config.watch {
        CARRIAGE_RETURN
    } else {
        NEW_LINE
    };

    if state.config.watch || state.config.log {
        loop {
            render_line(delimiter, &mut state);
            stdout().flush()?;
            thread::sleep(Duration::from_secs(state.config.interval));
        }
    } else {
        render_line(delimiter, &mut state);
    }
    Ok(())
}
