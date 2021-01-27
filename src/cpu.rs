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
use super::config::Config;
use futures_timer::Delay;
use heim::cpu;
use heim::units::ratio;
use std::cmp;
use std::time::Duration;

pub async fn usage(config: &Config) -> Result<String, heim::Error> {
    let measurement_1 = cpu::usage().await?;
    Delay::new(Duration::from_millis(100)).await;
    let measurement_2 = cpu::usage().await?;
    let num_cpu = cpu::logical_count().await? as f32;
    let usage = cmp::min(
        100,
        ((measurement_2 - measurement_1).get::<ratio::percent>() / num_cpu).round() as i32,
    );
    Ok(format!(
        "{}%",
        colorize(
            usage as f32,
            config.threshold_cpu_high,
            config.threshold_cpu_medium
        )
    ))
}

pub async fn frequency() -> Result<String, heim::Error> {
    Ok(format!(
        "{:4} MHz",
        (cpu::frequency().await?.current().value as f32 / 1000_000.0).round()
    ))
}
