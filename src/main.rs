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

use async_std;
use clap::Clap;
use futures_timer::Delay;
use std::io::stdout;
use std::io::Write;
use std::time::Duration;

use sysit::config::Config;
use sysit::cpu;
use sysit::memory;
use sysit::sensors;

use colored::control::set_override;

const NEW_LINE: char = '\n';
const CARRIAGE_RETURN: char = '\r';

async fn line(config: &Config) -> Result<String, heim::Error> {
    Ok(format!(
        "M:{} | C:{} @ {} | T:{}",
        memory::usage(config).await?,
        cpu::usage(config).await?,
        cpu::frequency().await?,
        sensors::temperature(config).await?
    ))
}

async fn render_line(delimiter: char, config: &Config) -> Result<(), heim::Error> {
    print!("{}{}", line(config).await?, delimiter);
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), heim::Error> {
    let config: Config = Config::parse();

    if config.colors {
        set_override(true);
    } else if config.no_colors {
        set_override(false);
    }

    if config.watch || config.log {
        loop {
            let delimiter = if config.log {
                NEW_LINE
            } else {
                CARRIAGE_RETURN
            };
            render_line(delimiter, &config).await?;
            stdout().flush()?;
            Delay::new(Duration::from_secs(config.interval)).await;
        }
    } else {
        render_line(NEW_LINE, &config).await?;
    }
    Ok(())
}
