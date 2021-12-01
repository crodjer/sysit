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

pub fn colorize(output: String, metric: f32, high: f32, mid: f32) -> String {
    let color = if metric > high {
        "red"
    } else if metric > mid {
        "yellow"
    } else {
        "green"
    };

    format!("{:<6}", output).color(color).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red() {
        assert_eq!(
            colorize("90%".to_string(), 90.0, 80.0, 50.0),
            "90%   ".red().to_string()
        );
    }

    #[test]
    fn test_yellow() {
        assert_eq!(
            colorize("FooBar".to_string(), 75.0, 80.0, 50.0),
            "FooBar".yellow().to_string()
        );
    }

    #[test]
    fn test_green() {
        assert_eq!(
            colorize("15.5 ms".to_string(), 15.0, 80.0, 50.0),
            "15.5 ms".green().to_string()
        );
    }
}
