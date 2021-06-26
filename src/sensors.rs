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
use sysinfo::{ComponentExt, System, SystemExt};

fn max_temperature(system: &System) -> Option<f32> {
    let mut max_temp: Option<f32> = None;

    for component in system.get_components() {
        let temp = component.get_temperature();

        max_temp = match max_temp {
            Some(current_max) => {
                if temp > current_max {
                    Some(temp)
                } else {
                    Some(current_max)
                }
            }
            None => Some(temp),
        };
    }

    max_temp
}

pub fn temperature(config: &Config, system: &System) -> String {
    max_temperature(system)
        .map(|t| {
            format!(
                "{}°C",
                colorize(t, config.threshold_temp_hot, config.threshold_temp_warm)
            )
        })
        .unwrap_or(String::from("N/A"))
}
