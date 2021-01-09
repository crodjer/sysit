# Sysit
System Sit, check on the system with a quick glance!

## About
Provides basic system information in a short amount of text, ideally
within 40 characters. Relies on [heim](https://heim-rs.github.io/) to
get all the relevant system information.

```
sysit 0.1
Rohan <crodjer@gmail.com>
Get basic system information in one line. For more information use --help

USAGE:
    sysit [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --log        Run in log mode. Will continuously append a row to standard output
    -V, --version    Prints version information
    -w, --watch      Run in watch mode. Will act as if running with the watch command

OPTIONS:
    -i, --interval <interval>    Specify update interval in seconds for watch/log mode [default: 1]

```

## Reasoning
When working headlessly with tmux or even when working on a desktop,
being able to quickly see basic system informatino can be useful.
There are various system monitors and even `btm` (also based on
`heim`), but you have to actively switch to them.

For instance, when overclocking the RPi, it makes sense to monitor the
CPU frequency and temperature. Inspired by [cpu-temp-speed](https://gitlab.manjaro.org/manjaro-arm/packages/community/cpu-temp-speed)
on Manjaro, which has been quite useful in tuning the overclock and
colling system configuration.

# Usage

## From the console
Simply type `sysit` for a quick glance at the system information.
```
rohan in mir in sysit on  master is 📦 v0.1.0 via 🦀 v1.49.0
at 17:45:13 ❯ sysit
M: 8% | C: 0% @ 1100 MHz | T: 54°C
```

This can also be used with a desktop manager's applets. For example,
Xfce's `genmon`.

## Continuous Monitoring
### Watch Mode
Works as if `watch sysit`. Can be used within tmux status line for
continuous monitoring. Eg:
```tmux.conf
set -g status-right '#[fg=yellow] #(sysit -wi 2) #[fg=colour235,bg=colour252,bold] %a %d %b %H:%M:%S #[fg=colour252,bg=colour238,nobold]#[fg=colour245,bg=colour238,bold]'
```
Watch mode with `sysit -wi 2` has a benefit of maintaining a single
process. Just using plain `sysit` command will also work, but that'd
mean tmux spawns a new process every time.

![sysit with tmux](assets/sysit-on-tmux.png?raw=true "sysit with tmux")

### Log Mode
At times it can be handy to log system stats, for instance, while
benchmarking.
```
rohan in mir in sysit on  master [?] is 📦 v0.1.0 via 🦀 v1.49.0
at 19:22:44 ❯ sysit -li 1
M: 11% | C: 0% @ 675 MHz | T: 55°C
M: 11% | C: 2% @ 600 MHz | T: 54°C
M: 11% | C: 87% @ 2100 MHz | T: 56°C
M: 11% | C: 100% @ 2100 MHz | T: 60°C
M: 11% | C: 100% @ 2100 MHz | T: 63°C
M: 11% | C: 100% @ 2100 MHz | T: 61°C
M: 11% | C: 0% @ 600 MHz | T: 59°C
```
