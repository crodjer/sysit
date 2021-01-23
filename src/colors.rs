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

use colored::*;

pub fn colorize(metric: f32, high: f32, mid: f32) -> String {
    let color = if metric > high {
        "red"
    } else if metric > mid {
        "yellow"
    } else {
        "green"
    };

    format!("{:.0}", metric as i32).color(color).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red() {
        assert_eq!(colorize(90.0, 80.0, 50.0), "90".red().to_string());
    }

    #[test]
    fn test_yellow() {
        assert_eq!(colorize(75.0, 80.0, 50.0), "75".yellow().to_string());
    }

    #[test]
    fn test_green() {
        assert_eq!(colorize(15.0, 80.0, 50.0), "15".green().to_string());
    }
}
