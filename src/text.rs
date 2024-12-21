use agb::{display::object::{Graphics, Sprite}, include_aseprite};

pub struct SpriteFont {
    spritesheet: &'static Graphics,
    chars: &'static [Sprite] // ascii 32-127, 96 chars, each 16x16 including the padding, where del's symbol is used for unknown chars
    // for reference that is, in this order:
    //~ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~�
    // excluding the first tilde, and the `del` char will be used
}

impl SpriteFont {
    pub fn load_font(fnt: &'static Graphics) -> Self {
        Self {
            spritesheet: fnt,
            chars: fnt.sprites()
        }
    }
}