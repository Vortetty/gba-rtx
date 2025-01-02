use core::mem;

use agb::{display::bitmap3::{self, Bitmap3}, input::{Button, ButtonController}};
use alloc::string::{String, ToString};

use crate::pixelara::{self, PIXELARA};

#[repr(i8)]
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum MenuSelection {
    SceneSelect=0,
    IterationsSelect,
    DepthSelect,
    ConfirmButton
}
#[repr(i8)]
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scenes {
    SPHERES
}

impl MenuSelection {
    pub fn next(&self) -> Self {
        unsafe {
            mem::transmute((self.clone() as u8).wrapping_add(1) % (mem::variant_count::<Self>() as u8))
        }
    }
    pub fn prev(&self) -> Self {
        unsafe {
            mem::transmute((self.clone() as u8).wrapping_sub(1) % (mem::variant_count::<Self>() as u8))
        }
    }
    pub fn from_i8(n: i8) -> Self {
        unsafe {
            mem::transmute(n)
        }
    }
}
impl Scenes {
    pub fn next(&self) -> Self {
        unsafe {
            mem::transmute((self.clone() as u8).wrapping_add(1) % (mem::variant_count::<Self>() as u8))
        }
    }
    pub fn prev(&self) -> Self {
        unsafe {
            mem::transmute((self.clone() as u8).wrapping_sub(1) % (mem::variant_count::<Self>() as u8))
        }
    }
    pub fn from_i8(n: i8) -> Self {
        unsafe {
            mem::transmute(n)
        }
    }
}

#[derive(Clone, Copy)]
pub struct RenderConfig {
    scene: Scenes,
    iters_per_pixel: u8,
    max_depth: u8
}

fn get_scene_name(scene: Scenes) -> String {
    match scene {
        Scenes::SPHERES => "Random Spheres",

        #[allow(unreachable_patterns)]
        _ => "Unimplemented"
    }.to_string()
}

//
// Renders the menu
// Characters are 8x8 and the screen is 240x160
//  so we get 30x20 characters
//  let the box be the image/description for each option
//  each character is represented by a character followed by a space for a more accurate spacing
//  title should be a new
//                        10  12  14  16  18  20  22  24  26  28
//     0 1 2 3 4 5 6 7 8 9  11  13  15  17  19  21  23  25  27  29
//    +------------------------------------------------------------+
//  0 |G B A - R T   C o n f i g u r a t i o n                     |
//  1 |                                                            |
//  2 |S c e n e :   < S p h e r e s >                             |
//  3 |I t e r s :   < 4 >                                         |
//  4 |D e p t h :   < 8 >                                         |
//  5 |                                                            |
//  6 |████████████████████████████████████████████████████████████|
//  7 |████████████████████████████████████████████████████████████|
//  8 |████████████████████████████████████████████████████████████|
//  9 |████████████████████████████████████████████████████████████|
// 10 |████████████████████████████████████████████████████████████|
// 11 |████████████████████████████████████████████████████████████|
// 12 |████████████████████████████████████████████████████████████|
// 13 |████████████████████████████████████████████████████████████|
// 14 |████████████████████████████████████████████████████████████|
// 15 |████████████████████████████████████████████████████████████|
// 16 |████████████████████████████████████████████████████████████|
// 17 |████████████████████████████████████████████████████████████|
// 18 |                                                            |
// 19 |C o n f i r m                                               |
//    +------------------------------------------------------------+
//
macro_rules! rFillText {
    ($text: expr) => {
        format!("{: <30}", $text)
    };
}
macro_rules! fmtOption {
    ($name: expr, $value: expr, $selected: expr) => {
        if $selected {
            format!("{: <30}", format!("{}: <{}>", $name, $value))
        } else {
            format!("{: <30}", format!("{}: {}", $name, $value))
        }
    };
}
fn render_menu(data: &RenderConfig, selection: &MenuSelection, bitmap: &mut Bitmap3) {
    //PIXELARA.print_str(match selection {
    //    MenuSelection::SceneSelect => "Scene Select    ",
    //    MenuSelection::IterationsSelect => "Iter Select     ",
    //    MenuSelection::DepthSelect => "Depth Select    ",
    //    MenuSelection::ConfirmButton => "Confirm Settings",
    //}, bitmap, 0, 0);
    PIXELARA.print_str_rel(rFillText!("GBA-RT Configuration"), bitmap, 0, 0);
    PIXELARA.print_str_rel(fmtOption!("Scene", get_scene_name(data.scene), *selection == MenuSelection::SceneSelect), bitmap, 0, 2);
    PIXELARA.print_str_rel(fmtOption!("Iters", data.iters_per_pixel, *selection == MenuSelection::IterationsSelect), bitmap, 0, 3);
    PIXELARA.print_str_rel(fmtOption!("Depth", data.max_depth, *selection == MenuSelection::DepthSelect), bitmap, 0, 4);
    PIXELARA.print_str_rel(rFillText!(if *selection == MenuSelection::ConfirmButton {
        "> Confirm Settings <"
    } else {
        "Confirm Settings"
    }), bitmap, 0, 19);
}

pub fn get_render_config(input: &mut ButtonController, bitmap: &mut Bitmap3) -> RenderConfig {
    let vblank = agb::interrupt::VBlank::get();

    let mut data = RenderConfig {
        iters_per_pixel: 4,
        scene: Scenes::from_i8(0),
        max_depth: 8
    };

    let mut menu_selection = MenuSelection::SceneSelect;

    bitmap.clear(0);
    render_menu(&data, &menu_selection, bitmap);

    loop {
        let mut one_true = true;
        agb::interrupt::VBlank::wait_for_vblank(&vblank);
        if input.is_just_pressed(Button::UP) {
            menu_selection = menu_selection.prev();
        } else if input.is_just_pressed(Button::DOWN) {
            menu_selection = menu_selection.next();
        } else if input.is_just_pressed(Button::RIGHT) {
            match menu_selection {
                MenuSelection::SceneSelect => {
                    data.scene = data.scene.next();
                },
                MenuSelection::IterationsSelect => {
                    if data.iters_per_pixel < 255 {
                        data.iters_per_pixel += 1;
                    }
                },
                MenuSelection::DepthSelect => {
                    if data.max_depth < 255 {
                        data.max_depth += 1;
                    }
                },
                MenuSelection::ConfirmButton => {}
            }
        } else if input.is_just_pressed(Button::LEFT) {
            match menu_selection {
                MenuSelection::SceneSelect => {
                    data.scene = data.scene.prev();
                },
                MenuSelection::IterationsSelect => {
                    if data.iters_per_pixel > 1 {
                        data.iters_per_pixel -= 1;
                    }
                },
                MenuSelection::DepthSelect => {
                    if data.max_depth > 1 {
                        data.max_depth -= 1;
                    }
                },
                MenuSelection::ConfirmButton => {}
            }
        } else if input.is_just_pressed(Button::A) {
            if menu_selection == MenuSelection::ConfirmButton {
                break
            }
        } else {
            one_true = false;
        }

        if one_true {
            render_menu(&data, &menu_selection, bitmap);
        }

        input.update();
    }

    return data;
}
