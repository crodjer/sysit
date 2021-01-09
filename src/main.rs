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
use futures_lite::stream::StreamExt;
use futures_timer::Delay;
use heim::cpu;
use heim::memory;
use heim::sensors;
use heim::units::ratio;
use std::io::stdout;
use std::io::Write;
use std::time::Duration;

const KELVIN_TO_CELSIUS: f32 = 273.15;

/// Get basic system information in one line.
/// For more information use --help
#[derive(Clap)]
#[clap(version = "0.1", author = "Rohan <crodjer@gmail.com>")]
struct Opts {
    /// Run in watch mode. Will act as if running with the watch command.
    #[clap(short, long)]
    watch: bool,

    /// Run in log mode. Will continuously append a row to standard output.
    #[clap(short, long)]
    log: bool,

    /// Specify update interval in seconds for watch/log mode.
    #[clap(short, long, default_value = "1")]
    interval: u64,
}

async fn mem_usage() -> Result<String, heim::Error> {
    let memory = memory::memory().await?;
    let total = memory.total().value as f32;
    let free = memory.available().value as f32;
    let usage = (100.0 * (total - free) / total).round();
    Ok(format!("{:.0}%", usage))
}

async fn cpu_usage() -> Result<String, heim::Error> {
    let measurement_1 = cpu::usage().await?;
    Delay::new(Duration::from_millis(100)).await;
    let measurement_2 = cpu::usage().await?;
    let num_cpu = cpu::logical_count().await? as f32;
    let usage = (measurement_2 - measurement_1).get::<ratio::percent>() / num_cpu;
    Ok(format!("{:.0}%", usage))
}

async fn cpu_freq() -> Result<String, heim::Error> {
    Ok(format!(
        "{} MHz",
        (cpu::frequency().await?.current().value as f32 / 1000_000.0).round()
    ))
}

async fn temperature() -> Result<String, heim::Error> {
    let mut sensor_data = sensors::temperatures().boxed_local();
    let mut max_temp: Option<f32> = None;

    while let Some(sensor) = sensor_data.next().await {
        if let Ok(sensor) = sensor {
            let temp = (sensor.current().value - KELVIN_TO_CELSIUS).round();
            max_temp = match max_temp {
                Some(current_max) => {
                    if temp > current_max {
                        Some(temp)
                    } else {
                        None
                    }
                }
                None => Some(temp),
            };
        }
    }

    Ok(max_temp
        .map(|t| format!("{}°C", t))
        .unwrap_or(String::from("N/A")))
}

async fn line() -> Result<String, heim::Error> {
    Ok(format!(
        "M: {} | C: {} @ {} | T: {}",
        mem_usage().await?,
        cpu_usage().await?,
        cpu_freq().await?,
        temperature().await?
    ))
}

async fn render_line() -> Result<(), heim::Error> {
    print!("{}", line().await?);
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), heim::Error> {
    let opts: Opts = Opts::parse();

    if opts.watch || opts.log {
        loop {
            render_line().await?;
            stdout().flush()?;
            Delay::new(Duration::from_secs(opts.interval)).await;
            if opts.log {
                print!("\n");
            } else {
                print!("\r");
            }
        }
    } else {
        render_line().await?;
    }
    Ok(())
}
