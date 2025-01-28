run_mesen: fix
	mesen2 output/gba-rt.gba

run: build
	cargo run --release

build:
	cargo build --release

fix: build
	-rm -r output
	-mkdir output
	agb-gbafix target/armv4t-none-eabi/release/gba-rt -o output/gba-rt.gba
