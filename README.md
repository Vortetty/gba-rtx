# GBA RTX

## A raytracer... on a gameboy

<sub><sup>(but it's a wip)</sup></sub>

### Why?

Well why not, 'nuff said.

<sub>Not a good enough answer?</sub>

<sub><sup>I saw someone else made a raytracer but it was somewhat bad, i wanted one that looked nice so i made this :3</sup></sub>

<sub><sup><sub><sup>... Really tho know others have written ones before that look nice tbh i am just bored. Sue me lol</sup></sub></sup></sub>

## License

[![CC0](https://licensebuttons.net/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0/)  
To the extent possible under law, [Kali/Wintersys/Vortetty](https://github.com/Vortetty/gba-rtx) has waived all copyright and related or neighboring rights to GBA RTX. This work is published from: United States.

### Why would i do this?

*Why not lmao*, really though this is just a side project, i stand to gain nothing from this and a public license is the best way to do it. This is the closest i can get to public domain that works in 99.99999% of places so, y'all enjoy it :3

## Building

### Prerequisites

You will need the following installed in order to build and run this project:

* A recent version of `rustup`. See the [rust website](https://www.rust-lang.org/tools/install) for instructions for your operating system
* `arm-none-eabi-binutils` for assembling and linking
    * Windows: [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).
        Make sure you select "Add path to environment variable" during the install
    * Debian and derivatives (e.g. Ubuntu, raspberry pi OS, linux mint): `sudo apt install binutils-arm-none-eabi`
    * Arch linux and derivatives: `sudo pacman -S arm-none-eabi-binutils`

You will also want to install an emulator. The best support in agb is with [mgba](https://mgba.io), with
`println!` support via `agb::println!` but any emulator should work. You'll get the best experience if
`mgba-qt` is in your `PATH`.

If you want to run your game on real hardware, you will also need to install `agb-gbafix` which you can do after installing
rust with the following: `cargo install agb-gbafix`. This is not required if you are only running your game in an emulator.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
cargo build
```

or in release mode (recommended for the final version to ship to players)

```sh
cargo build --release
```

The resulting file will be in `target/thumbv4t-none-eabi/debug/<your game>` or `target/thumbv4t-none-eabi/release/<your game>` depending on
whether you did a release or debug build.

If you have `mgba-qt` in your path, you will be able to run your game with

```sh
cargo run
```

or in release mode

```sh
cargo run --release
```

## Starting development

You can find the documentation for agb [here](https://docs.rs/agb/latest/agb/).

You may also want to change the package name and version in `Cargo.toml` before you start.

## Shipping a .gba file for real hardware

To make a game run on real hardware, you will need to convert the built file into a file suitable for
running on the real thing.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/<your game> -o <your game>.gba
```