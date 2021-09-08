# mp (Make Pair with this command)
This script assign members to make pair/mob programming team

## Requirement

-   Install [Rust](https://www.rust-lang.org/tools/install)

## Install

```shell
./install
```

## Usage
You can assign pair/mob members randomly with the command.

```shell
➜ mp assign Walter,Ian,Gabe,Hiro,Ben,Charles
Team A: Ian Ben
Team B: Walter Gabe
Team C: Charles Hiro

Do you save the result to history? [y/n]:
y
Saved the result
```

```shell
➜ mp history
[2021-06-04 02:50:54.666529 UTC] Team A: Ian Ben Team B: Walter Gabe Team C: Charles Hiro
```
