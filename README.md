# mp (Make Pair with this command)
This script assign members to make pair/mob programming team

## Install
```shell
$ cd mp
$ cargo build --release
$ cp target/release/mp /usr/local/bin
```

## Usage
You can assign pair/mob members randomly with the command.
```shell
$ mp Walter,Ian,Gabe,Hiro,Ben,Charles
Team A: Gabe Hiro
Team B: Charles Ian
Team C: Walter Ben
```

```shell
$ mp Walter,Ian,Gabe,Hiro,Ben,Charles,Ken
Team A: Walter Hiro Gabe
Team B: Ben Ken
Team C: Charles Ian
```