use agb::display::bitmap3::Bitmap3;

pub struct Font<const XSIZE: usize, const YSIZE: usize, const CHARCNT: usize> {
    palette: &'static [u16], // Up to 256 colors
    chars: &'static [[[u8; YSIZE]; XSIZE]; CHARCNT] // chars 32-255 supported
    // for reference that is, in this order:
    //~ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~�
    // excluding the first tilde, and the `del` char will be used
}

impl<const XSIZE: usize, const YSIZE: usize, const CHARCNT: usize> Font<XSIZE, YSIZE, CHARCNT> {
    pub const fn new(palette: &'static [u16], chars: &'static [[[u8; YSIZE]; XSIZE]; CHARCNT]) -> Self {
        Self {
            palette: palette,
            chars: chars
        }
    }

    pub fn print_nth(self: &Self, chr: u8, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) {
        for x in 0..XSIZE {
            let dx = x + pos_x;
            for y in 0..YSIZE {
                bitmap.draw_point(dx as i32, (y + pos_y) as i32, self.palette[self.chars[chr as usize][XSIZE-1-x][y] as usize]);
            }
        }
    }
    pub fn print_char(self: &Self, chr: u8, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) {
        self.print_nth(chr.wrapping_sub(32).min(CHARCNT as u8 - 1), bitmap, pos_x, pos_y);
    }
    pub fn print_str(self: &Self, text: impl AsRef<str>, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) {
        let mut pos_x = pos_x;
        let mut pos_y = pos_y;
        for chr in text.as_ref().chars() {
            if pos_x + XSIZE > 240 {
                pos_y += YSIZE;
                pos_x = 0;
            }
            self.print_char(chr as u8, bitmap, pos_x, pos_y);
            pos_x += XSIZE;
        }
    }
    // Prints a string but interprets the coords as a grid of tiles of XSIZE*YSIZE
    pub fn print_str_rel(self: &Self, text: impl AsRef<str>, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) {
        self.print_str(text, bitmap, pos_x * XSIZE, pos_y * YSIZE);
    }
}

//
// Example code for a font
//

// use crate::text::Font;
//
// const XSIZE: usize = 16;
// const YSIZE: usize = 16;
//
// static palette: [u16; 2] = [
//     // Valid gba colors here, 2 of them
// ];
//
// static chars: [[[u8; YSIZE]; XSIZE]; 96] = [
//     // each image as a 16x16 grid. must be indexed as chars[x][y]
// ];
//
// pub static FONT_NAME: Font<XSIZE, YSIZE> = Font::new(&palette, &chars);
//
