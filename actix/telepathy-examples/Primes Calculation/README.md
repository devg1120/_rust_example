# Primes Calculation

A distributed [Actix-Telepathy](https://github.com/wenig/actix-telepathy) application that calculates all primes up to a given number by distributing the search space to a pool of workers.

## Usage

### Help

```shell
cargo r -- --help

> Prime Calculation 0.1.0
> Prime number calculation
> 
> USAGE:
>     telepathy-pc --host <host> --workers <workers> <SUBCOMMAND>
> 
> FLAGS:
>         --help       Prints help information
>     -V, --version    Prints version information
> 
> OPTIONS:
>     -h, --host <host>
>     -w, --workers <workers>
> 
> SUBCOMMANDS:
>     help    Prints this message or the help of the given subcommand(s)
>     main
>     sub
```

#### main

```shell
cargo r -- main --help

> telepathy-pc-main 0.1.0
> 
> USAGE:
>     telepathy-pc --host <host> --workers <workers> main --end <end> --nodes <nodes> --start <start>
> 
> FLAGS:
>     -h, --help       Prints help information
>     -V, --version    Prints version information
> 
> OPTIONS:
>     -e, --end <end>
>     -n, --nodes <nodes>
>     -s, --start <start>
```

#### sub

```shell
cargo r -- main --help

> telepathy-pc-sub 0.1.0
> 
> USAGE:
>     telepathy-pc --host <host> --workers <workers> sub --mainhost <mainhost>
> 
> FLAGS:
>     -h, --help       Prints help information
>     -V, --version    Prints version information
> 
> OPTIONS:
>     -m, --mainhost <mainhost>
```


### Run
This example can be run on one computer, that will act as 2 cluster nodes.

#### Cluster node 1
```shell
RUST_LOG=info cargo r -- --host 127.0.0.1:1992 --workers 2 main --nodes 2 --start 1 --end 100

> ...
> [2023-01-02T09:14:38Z INFO  telepathy_pc::actors::profiler] Found 25 primes, with 97 as maximum
> ...
```

#### Cluster node 2 (in different terminal window)
```shell
RUST_LOG=info cargo r -- --host 127.0.0.1:1993 --workers 2 sub --mainhost 127.0.0.1:1992

> ...
```
