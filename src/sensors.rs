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
use futures_lite::stream::StreamExt;
use heim::sensors;

const KELVIN_TO_CELSIUS: f32 = 273.15;

async fn max_temperature() -> Result<Option<f32>, heim::Error> {
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

    Ok(max_temp)
}

pub async fn temperature(config: &Config) -> Result<String, heim::Error> {
    Ok(max_temperature()
        .await?
        .map(|t| {
            format!(
                "{}°C",
                colorize(t, config.threshold_temp_hot, config.threshold_temp_warm)
            )
        })
        .unwrap_or(String::from("N/A")))
}
