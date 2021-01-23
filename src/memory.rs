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
use heim::memory;

pub async fn usage() -> Result<String, heim::Error> {
    let memory = memory::memory().await?;
    let total = memory.total().value as f32;
    let free = memory.available().value as f32;
    let usage = (100.0 * (total - free) / total).round();
    Ok(format!("{}%", colorize(usage, 80.0, 50.0)))
}
