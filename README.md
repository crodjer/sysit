# Sysit
System Sit, check on the system with a quick glance!

[![Crate](https://img.shields.io/crates/v/sysit.svg)](https://crates.io/crates/sysit)
[![Build](https://github.com/crodjer/sysit/workflows/CI/badge.svg?branch=main)](https://github.com/crodjer/sysit/actions?workflow=CI)

## About
System resources overview within 50 characters. Relies on
[sysinfo](https://github.com/GuillaumeGomez/sysinfo) to get all the relevant system
information.

![sysit with tmux](https://raw.githubusercontent.com/crodjer/sysit/main/assets/sysit-on-tmux.png)

## Install
If you are on Arch, install via Aur: https://aur.archlinux.org/packages/sysit-bin/

On other platforms, you can use the install script, which will install
a [pre-built binary](https://github.com/crodjer/sysit/releases/latest).
To install at `/usr/local/bin/`:
```
curl -s https://raw.githubusercontent.com/crodjer/sysit/main/scripts/install.sh | sudo bash
```
Or, to install at a location of your choice, say `~/.local/bin`:
```
curl -s https://raw.githubusercontent.com/crodjer/sysit/main/scripts/install.sh | bash -s ~/.local/bin
```

You can always use cargo if your platform isn't supported:
```
cargo install sysit
```


## Reasoning
The ability to quickly see basic system information without needing a
context switch can be useful. `sysit` is easy to incorporate in the
various status bars - such as that of `tmux`, `i3`/`sway` etc.

## Understanding the output
  Memory Usage
  CPU Information (usage and optionally frequency)
  Temperature for the hottest sensor
  Network Ping

# Usage

## From the console
Simply type `sysit` for a quick glance at the system information.
```
sysit on  main is 📦 v0.6.0 via 🦀 v1.56.1
at 18:43:42 ❯ sysit
 21%   5%    45°C    12.0 ms
```

This can also be used with a desktop manager's applets. For example,
Xfce's `genmon`.

## Continuous Monitoring
### Watch Mode
Works as if `watch sysit`. Can be used within tmux status line for
continuous monitoring. Eg:
```tmux.conf
set -g status-right '#[fg=black,bg=blue] #(sysit -wi 2) '
```
Watch mode with `sysit -wi 2` has a benefit of maintaining a single
process. Just using plain `sysit` command will also work, but that'd
mean tmux spawns a new process every time.

### Log Mode
At times it can be handy to log system stats, for instance, while
benchmarking.
```
sysit on  main is 📦 v0.6.0 via 🦀 v1.56.1
at 18:45:26 ❯ sysit -lf
 21%   5%   @2.9 GHz   44°C    9.91 ms
 21%   2%   @2.1 GHz   44°C    8.43 ms
 21%   3%   @2.2 GHz   46°C    14.4 ms
 21%   1%   @3.8 GHz   46°C    139 ms
 24%   63%  @4.1 GHz   55°C    17.9 ms
 25%   10%  @4.0 GHz   48°C    354 ms
 26%   10%  @4.1 GHz   45°C    472 ms
```

## Help
```
sysit

Get system resources overview in 50 characters
For usage details, try --help

Understanding the output:
   Memory Usage
   CPU Information (usage and optionally frequency)
   Temperature for the hottest sensor
   Network Ping

USAGE:
    sysit [OPTIONS]

OPTIONS:
    -c, --colors
            force output to be always colorized

    -f, --frequency
            show CPU frequency

    -h, --help
            Print help information

    -i, --interval <INTERVAL>
            update interval in seconds for watch/log mode

            [default: 1]

    -l, --log
            run in log mode (will continuously append a row to standard output)

        --no-colors
            force output to be never colorized

        --ping-host <PING_HOST>
            host to use for testing the ping

            [default: 1.0.0.1]

        --threshold-cpu-high <THRESHOLD_CPU_HIGH>
            the threshold for high cpu usage (higher values will be rendered in red)

            [default: 80.0]

        --threshold-cpu-medium <THRESHOLD_CPU_MEDIUM>
            the threshold for medium cpu usage (higher values will be rendered in yellow)

            [default: 50.0]

        --threshold-memory-high <THRESHOLD_MEMORY_HIGH>
            the threshold for high memory usage (higher values will be rendered in red)

            [default: 80.0]

        --threshold-memory-medium <THRESHOLD_MEMORY_MEDIUM>
            the threshold for medium memory usage (higher values will be rendered in yellow)

            [default: 50.0]

        --threshold-temp-hot <THRESHOLD_TEMP_HOT>
            the threshold for high temperature (higher values will be rendered in red)

            [default: 75.0]

        --threshold-temp-warm <THRESHOLD_TEMP_WARM>
            the threshold for warm temperature (higher values will be rendered in yellow)

            [default: 55.0]

    -V, --version
            Print version information

    -w, --watch
            run in watch mode (as if running with the watch command)
```
