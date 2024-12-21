use core::{intrinsics::wrapping_sub, mem};

use agb::{display::{object::{Graphics, Size, Sprite}, palette16::Palette16}, include_aseprite};

pub struct SpriteFont {
    spritesheet: &'static Graphics,
    chars: &'static [Sprite] // ascii 32-127, 96 chars, each 16x16 including the padding, where del's symbol is used for unknown chars
    // for reference that is, in this order:
    //~ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~�
    // excluding the first tilde, and the `del` char will be used
}

pub struct AccessibleSprite {
    pub palette: &'static Palette16,
    pub data: &'static [u8],
    pub size: Size,
}

impl SpriteFont {
    pub fn load_font(fnt: &'static Graphics) -> Self {
        Self {
            spritesheet: fnt,
            chars: fnt.sprites()
        }
    }

    pub fn printChar(self: Self, chr: u8) {
        let mut chr = chr.wrapping_sub(32).min(96); // Offset into the space of our font and ensure the max number is 96
        let sprt: &AccessibleSprite = unsafe {
            mem::transmute(&self.chars[chr as usize])
        };

    }
}