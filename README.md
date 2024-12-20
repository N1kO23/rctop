# What is it?
[![RELEASE](https://github.com/N1kO23/rctop/actions/workflows/rust.yml/badge.svg)](https://github.com/N1kO23/rctop/actions/workflows/rust.yml)
[![CI](https://github.com/N1kO23/rctop/actions/workflows/ci.yml/badge.svg)](https://github.com/N1kO23/rctop/actions/workflows/ci.yml)

RCTOP is a simple WIP system monitoring app that runs purely on terminal and doesn't feature GUI. One can compare it to htop, but more stripped down. RCTOP is fully coded in Rust.

## Notable features

- Lightweight
- Small filesize
- Shows CPU usage (system, user, interrupts, etc)
- Shows RAM usage
- Shows mounted drives and how full they are (not implemented yet)
- Shows battery level, if supported
- Shows CPU temps, if supported (not implemented yet)

## Early version of the UI
 ![Very early development UI](https://cdn.discordapp.com/attachments/881632596298170399/907643426873430057/unknown.png)

# Installation

To compile and run the program from source code, one needs to have Rust installed, it can be downloaded from [here](https://www.rust-lang.org/tools/install "Link to rust installer download page").

### Manually cloning with GitHub

1. Clone the repository `git clone https://github.com/N1kO23/rctop/`
2. Go to the cloned directory `cd ./rctop`
3. Build dev version `cargo build` or build optimized release version `cargo build --release`
4. A folder called `target` should be generated and based on build parameters the compiled binary is in `/target/debug` or `/target/release`

### Other installation options will be available later

# TODO

- Add pagefile usage
- Add uptime indicator
- Add drive list with usages
- Add battery indicator
- Add cpu temp indicator
- Add network throughput indicator
- Add tabs for some fields to show extra information (example. cpu details)
- Make the termial look actually good
- Make keyboard interrupt handler, rn handles only ctrl+c
- Optimize terminal view update
- Multithread the thing
- Launch arg handling for different things such as refresh rate
