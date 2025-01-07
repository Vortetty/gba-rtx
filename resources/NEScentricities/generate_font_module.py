#!/bin/python3
from PIL import Image
import numpy as np

FONT_NAME = "NEScentricities"  # will load FONT_NAME.png and then read the characters left-to-right in the order listed below. 16x16
CHARACTERS = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
BACKGROUND_COLOR_RGB = [0, 0, 0]

im_in = Image.open(FONT_NAME + ".png").convert("RGBA").rotate(-90, expand=True)
im1 = Image.new("RGBA", im_in.size, tuple(BACKGROUND_COLOR_RGB) + (255,))

im1.paste(im_in, mask=im_in)

im_indexed_backgrounded = im1.convert("RGB").quantize(colors=256)

imarr: np.array = np.array(im_indexed_backgrounded)
imarr.resize((imarr.shape[0] // 16, 16, 16))


imagestrings = []
for i in imarr:
    out = "    [\n"
    im = Image.fromarray(i).rotate(180)
    for k in np.array(im):
        out += "        " + str([int(i) for i in list(k)]) + ",\n"
    imagestrings.append(out + "    ],\n")

if imagestrings.__len__() != CHARACTERS.__len__():
    raise ValueError(f"Wrong number of images ({imagestrings.__len__()}) compared to characters ({CHARACTERS.__len__()}). Expected equal counts")

with open(f"{FONT_NAME.lower()}.rs", 'w') as f:
    f.write("//\n")
    f.write(f"// {FONT_NAME} (c) 2024 by Kali H. is licensed under CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0/)\n")
    f.write("//\n")
    f.write("use super::text::Font;\n")
    f.write("\n")
    f.write("const XSIZE: usize = 16;\n")
    f.write("const YSIZE: usize = 16;\n")
    f.write("\n")
    f.write(f"static palette: [u16; {im_indexed_backgrounded.palette.colors.__len__()}] = [\n")
    for i in im_indexed_backgrounded.palette.colors:
        f.write(f"    0b0_{str(bin(i[2] >> 3)[2:]).rjust(5, "0")}_{str(bin(i[1] >> 3)[2:]).rjust(5, "0")}_{str(bin(i[0] >> 3)[2:]).rjust(5, "0")},\n")
    f.write("];\n")
    f.write("\n")
    f.write(f"static chars: [[[u8; YSIZE]; XSIZE]; {imagestrings.__len__()}] = [\n")
    for (i, k) in zip(CHARACTERS, imagestrings):
        f.write(f"    // Char: {i}\n")
        f.write(k)
    f.write("];\n")
    f.write("\n")
    f.write(f"pub static {FONT_NAME.upper()}: Font<XSIZE, YSIZE, {imagestrings.__len__()}> = Font::new(&palette, &chars);\n")

print(f"""Generated font: {FONT_NAME.lower()}.rs
    Characters: {CHARACTERS.__len__()} (\033[3m\033[7m{CHARACTERS}\033[0m)
        Colors: { ''.join('\n          - ({}, {}, {})'.format(*i) for i in im_indexed_backgrounded.palette.colors) }""")