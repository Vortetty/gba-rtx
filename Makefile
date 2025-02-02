COLOUR_RED=\033[0;31m
COLOUR_YEL=\033[0;33m
COLOUR_RES=\033[0m

run_mesen: fix
	mesen2 output/gba-rt.gba

run: build
	@echo -e "$(COLOUR_RED)----------------------------------------------------------$(COLOUR_RES)"
	@echo -e "$(COLOUR_YEL) Running via MGBA may be more convenient on some systems, $(COLOUR_RES)"
	@echo -e "$(COLOUR_YEL)         however the emulation is less accurate.          $(COLOUR_RES)"
	@echo -e "$(COLOUR_YEL)  be aware of this as colors and times may be incorrect.  $(COLOUR_RES)"
	@echo -e "$(COLOUR_RED)----------------------------------------------------------$(COLOUR_RES)"
	@echo -e "$(COLOUR_YEL)                 Press enter to continue                  $(COLOUR_RES)"
	@echo -e "$(COLOUR_RED)----------------------------------------------------------$(COLOUR_RES)"
	@read
	cargo run --release

build:
	cargo build --release

fix: build
	-rm -r output
	-mkdir output
	agb-gbafix target/armv4t-none-eabi/release/gba-rt -o output/gba-rt.gba
