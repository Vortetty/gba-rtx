rd /s /q output
mkdir output
cargo build --release
agb-gbafix target/armv4t-none-eabi/release/gba-rt -o output/gba-rt.gba
mgba output/gba-rt.gba