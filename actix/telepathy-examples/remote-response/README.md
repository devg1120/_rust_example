# Remote Response

A distributed [Actix-Telepathy](https://github.com/wenig/actix-telepathy) application that sends a message and waits for a remote response before handling other messages.

## Usage

### Help
```shell
cargo r -- --help

> Usage: remote-response [OPTIONS] --host <HOST>
> 
> Options:
>       --host <HOST>
>       --seed <SEED>
>   -h, --help         Print help
```

### Run
This example can be run on one computer, that will act as 2 cluster nodes.

#### Cluster node 1
```shell
RUST_LOG=debug cargo r -- --host 127.0.0.1:1992

> ...
```

#### Cluster node 2 (in different terminal window)
```shell
RUST_LOG=debug cargo r -- --host 127.0.0.1:1993 --seed 127.0.0.1:1992

> ...
```