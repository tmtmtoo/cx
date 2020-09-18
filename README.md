# _cx_

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
    -i, --interval <interval>    execution interval (sec) [default: 0.1]
    -m, --max <max>              maximum number of retries

ARGS:
    <COMMAND>...    command and options
```

### example
```bash
$ cx retry -m 3 -i 2 -- your command that may fail && echo succeeded || echo failed
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
    -i, --interval <interval>    execution interval (sec) [default: 0.1]
    -l, --limit <limit>          re-execution limit

ARGS:
    <COMMAND>...    command and options
```

### example
```bash
$ cx supervise -l 3 -i 2 -- echo abc
```
