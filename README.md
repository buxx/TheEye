# TheEye

An exemple project for Rust formation.

## What the project do

TheEye is a program which permit to watch a specific metric of a linux process. 

For examples, for the `top`command (start an `top` command in some other terminal), see cpu usage changes :

    cargo run --bin process -- top cpu

To build production bin :

    cargo build --release

bin file will be available at `target/release/process`

## Sources

* `eye-cli` CLI bin tool using `eye-lib`
* `eye-lib` lib of the project

Start by take a look of `eye-cli/src/bin/process.rs` and explore source code from here.