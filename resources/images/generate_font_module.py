#!/bin/python3
from PIL import Image
import numpy as np
import os

SIZE = (144, 96)
CHARACTERS = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
BACKGROUND_COLOR_RGB = [0, 0, 0]

images = [ # Please never use code like this ;-;
    [
        (
            z:=Image.new("RGBA", SIZE, tuple(BACKGROUND_COLOR_RGB) + (255,))
        ),
        z.paste(
            (
                x:=Image.open("input/" + i).convert("RGBA").resize(SIZE)
            ),
            mask=x
        )
    ][0].convert("RGB") for i in filter(
        lambda y:y.endswith(".png") or y.endswith(".jpg"),
        os.listdir("input")
    )
]

combinedImages = np.array(images[0])

for image in images[1:]:
    combinedImages = np.concatenate([combinedImages, np.array(image)])

flattened = combinedImages.flatten()
for i, px in enumerate(flattened):
    flattened[i] = (px >> 3) << 3

combinedImages = flattened.reshape(combinedImages.shape)

im_indexed_backgrounded = Image.fromarray(combinedImages, "RGB").quantize(colors=256)

imarr: np.array = np.array(im_indexed_backgrounded)
imarr.resize((images.__len__(), SIZE[1], SIZE[0]))

imagestrings = []
for i in imarr:
    out = "    [\n"
    im = Image.fromarray(i, "P").rotate(90, expand=True)
    for k in np.array(im):
        out += "        " + str([int(i) for i in list(k)]) + ",\n"
    imagestrings.append(out + "    ],\n")

with open(f"images.rs", 'w') as f:
    f.write("use super::text::Font;\n")
    f.write("\n")
    f.write(f"const XSIZE: usize = {SIZE[0]};\n")
    f.write(f"const YSIZE: usize = {SIZE[1]};\n")
    f.write("\n")
    f.write(f"static palette: [u16; {im_indexed_backgrounded.palette.colors.__len__()}] = [\n")
    for i in im_indexed_backgrounded.palette.colors:
        f.write(f"    0b0_{str(bin(i[2] >> 3)[2:]).rjust(5, "0")}_{str(bin(i[1] >> 3)[2:]).rjust(5, "0")}_{str(bin(i[0] >> 3)[2:]).rjust(5, "0")},\n")
    f.write("];\n")
    f.write("\n")
    f.write(f"static chars: [[[u8; YSIZE]; XSIZE]; {imagestrings.__len__()}] = [\n")
    for (i, k) in zip(filter(lambda y:y.endswith(".png") or y.endswith(".jpg"),os.listdir("input")), imagestrings):
        f.write(f"    // File: {i}\n")
        f.write(k)
    f.write("];\n")
    f.write("\n")
    f.write(f"pub static IMAGES: Font<XSIZE, YSIZE, {imagestrings.__len__()}> = Font::new(&palette, &chars);\n")

print(f"""Generated font: images.rs
    Characters: {images.__len__()} (\033[3m\033[7m{list(filter(lambda y:y.endswith(".png") or y.endswith(".jpg"),os.listdir("input")))}\033[0m)
        Colors: { ''.join('\n          - ({}, {}, {})'.format(*i) for i in im_indexed_backgrounded.palette.colors) }""")