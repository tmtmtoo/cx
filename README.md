# _cx_  ![Test](https://github.com/tmtmtoo/cx/workflows/Test/badge.svg) ![Release](https://github.com/tmtmtoo/cx/workflows/Release/badge.svg)
A simple command executor utility.

```
Command eXecutor

USAGE:
    cx <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    retry        Retry command execution until successful.
    supervise    Supervise command execution.
```

## Retry
```
Retry command execution until successful.

USAGE:
    cx retry [OPTIONS] [COMMAND]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>          maximum number of retry counts
    -i, --interval <interval>    execution interval (sec) [default: 0.1]

ARGS:
    <COMMAND>...    command and options
```

### example
```bash
$ cx retry -c 3 -i 2 -- your command that may fail && echo succeeded || echo failed
```

## Supervise
```
Supervise command execution.

USAGE:
    cx supervise [OPTIONS] [COMMAND]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>          re-execution limit counts
    -i, --interval <interval>    execution interval (sec) [default: 0.1]

ARGS:
    <COMMAND>...    command and options
```

### example
```bash
$ cx supervise -c 3 -i 2 -- echo abc
```

## TODO
- [ ] feature: parallel
- [ ] build: armv7-unknown-linux-gnueabihf
