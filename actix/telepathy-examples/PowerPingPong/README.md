# PowerPingPong

A distributed [Actix-Telepathy](https://github.com/wenig/actix-telepathy) application that sends byte messages back and forth but every time increases the message size by the power of 2.

## Usage

### Help
```shell
cargo r -- --help

> Power Ping Pong
> 
> USAGE:
>     powerpingpong --host <host> --steps <steps> <SUBCOMMAND>
> 
> FLAGS:
>         --help       Prints help information
>     -V, --version    Prints version information
> 
> OPTIONS:
>     -h, --host <host>
>     -s, --steps <steps>
> 
> SUBCOMMANDS:
>     help    Prints this message or the help of the given subcommand(s)
>     main
>     sub
```

### Run
This example can be run on one computer, that will act as 2 cluster nodes.

#### Cluster node 1
```shell
RUST_LOG=info cargo r -- --host 127.0.0.1:1992 --steps 8 main

> ...
```

#### Cluster node 2 (in different terminal window)
```shell
RUST_LOG=info cargo r -- --host 127.0.0.1:1993 --steps 8 sub --mainhost 127.0.0.1:1992

> ...
```