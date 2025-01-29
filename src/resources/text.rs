use agb::{display::bitmap3::Bitmap3, sound::mixer::Mixer};

pub struct Font<const XSIZE: usize, const YSIZE: usize, const CHARCNT: usize> {
    palette: &'static [u16], // Up to 256 colors
    chars: &'static [[[u8; YSIZE]; XSIZE]; CHARCNT] // chars 32-255 supported, or 0-255 if using print_nth
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
        let chr_ref = &(self.chars[chr as usize]);

        for (x, x_array) in chr_ref.iter().rev().enumerate() {
            let dx = x + pos_x; // offset the x position
            for (y, y_val) in x_array.iter().enumerate() {
                bitmap.draw_point(dx as i32, (y + pos_y) as i32, self.palette[*y_val as usize]); // Gba's x/y is bottom left, the font's is top left. flip the y coord and then get that color from the palette
            }
        }
    }
    
    pub fn print_char(self: &Self, chr: u8, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) {
        self.print_nth(chr.wrapping_sub(32).min(CHARCNT as u8 - 1), bitmap, pos_x, pos_y); // Offset the char by 32 since " " is the first char in the list
    }
    
    pub fn print_str(self: &Self, text: impl AsRef<str>, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) { // Iterates a string and prints, with rudimentary line breaking!
        let mut pos_x = pos_x;
        let mut pos_y = pos_y;
        for chr in text.as_ref().chars() {
            if pos_x + XSIZE > 240 { // Break line automagically :3
                pos_y += YSIZE;
                pos_x = 0;
            }
            self.print_char(chr as u8, bitmap, pos_x, pos_y);
            pos_x += XSIZE; // move right
        }
    }
    
    pub fn print_str_rel(self: &Self, text: impl AsRef<str>, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize) { // Prints a string but interprets the coords as a grid of tiles of XSIZE*YSIZE
        self.print_str(text, bitmap, pos_x * XSIZE, pos_y * YSIZE);
    }


    
    pub fn print_nth_music(self: &Self, chr: u8, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize, mixer: &mut Mixer) {
        let chr_ref = &(self.chars[chr as usize]);

        for (x, x_array) in chr_ref.iter().rev().enumerate() {
            let dx = x + pos_x; // offset the x position
            for (y, y_val) in x_array.iter().enumerate() {
                bitmap.draw_point(dx as i32, (y + pos_y) as i32, self.palette[*y_val as usize]); // Gba's x/y is bottom left, the font's is top left. flip the y coord and then get that color from the palette
            }
            mixer.frame();
        }
    }
    
    pub fn print_str_music(self: &Self, text: impl AsRef<str>, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize, mixer: &mut Mixer) { // Iterates a string and prints, with rudimentary line breaking!
        let mut pos_x = pos_x;
        let mut pos_y = pos_y;
        for chr in text.as_ref().chars() {
            if pos_x + XSIZE > 240 { // Break line automagically :3
                pos_y += YSIZE;
                pos_x = 0;
            }
            self.print_char(chr as u8, bitmap, pos_x, pos_y); // Since this shouldn't be used for anything large like print_nth_music will be, we are not really worried about the music being called every line rendered like we are for images which use print_nth_music
            pos_x += XSIZE; // move right
            mixer.frame();
        }
    }
    
    pub fn print_str_rel_music(self: &Self, text: impl AsRef<str>, bitmap: &mut Bitmap3, pos_x: usize, pos_y: usize, mixer: &mut Mixer) { // Prints a string but interprets the coords as a grid of tiles of XSIZE*YSIZE
        self.print_str_music(text, bitmap, pos_x * XSIZE, pos_y * YSIZE, mixer);
    }
}

//
// Example code for a font
//

// use super::text::Font;
//
// const XSIZE: usize = 16;
// const YSIZE: usize = 16;
//
// static PALETTE: [u16; 2] = [
//     // Valid gba colors here, 2 of them
// ];
//
// static CHARS: [[[u8; YSIZE]; XSIZE]; 96] = [
//     // each image as a 16x16 grid. must be indexed as chars[x][y]
// ];
//
// pub static FONT_NAME: Font<XSIZE, YSIZE> = Font::new(&PALETTE, &CHARS);
//
