use async_std;
use futures_lite::stream::StreamExt;
use futures_timer::Delay;
use heim::cpu;
use heim::memory;
use heim::sensors;
use heim::units::ratio;
use std::time::Duration;

const KELVIN_TO_CELSIUS: f32 = 273.15;

async fn mem_usage() -> Result<String, heim::Error> {
    let memory = memory::memory().await?;
    let total = memory.total().value as f32;
    let free = memory.available().value as f32;
    let usage = (100.0 * (total - free) / total).round();
    Ok(format!("{:.0}%", usage))
}

async fn cpu_usage() -> Result<String, heim::Error> {
    let measurement_1 = cpu::usage().await?;
    Delay::new(Duration::from_millis(150)).await;
    let measurement_2 = cpu::usage().await?;
    let usage = (measurement_2 - measurement_1).get::<ratio::percent>();
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

#[async_std::main]
async fn main() -> Result<(), heim::Error> {
    println!("{}", line().await?);
    Ok(())
}
