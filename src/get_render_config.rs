use core::mem;

use agb::{display::bitmap3::{self, Bitmap3}, input::{Button, ButtonController}, sound::mixer::Mixer};
use alloc::{string::{String, ToString}, vec::Vec};

use crate::{images::IMAGES, pixelara::{self, PIXELARA}};

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
//                         10  12  14  16  18  20  22  24  26  28
//     0 1 2 3 4 5 6 7 8 9   11  13  15  17  19  21  23  25  27  29
//    +------------------------------------------------------------+
//  0 |G B A - R T   C o n f i g u r a t i o n                     |
//  1 |                                                            |
//  2 |S c e n e :   < S p h e r e s >                             |
//  3 |I t e r s :   < 4 >                                         |
//  4 |D e p t h :   < 8 >                                         |
//  5 |                                                            |
//  6 |████████████@@%******************************%@@████████████|
//  7 |████████████@+   +%@@@@@@@@@@@@@@@@@@@@@@%+   =@████████████|
//  8 |████████████@:  =@@@@@@@@@@@@@@@@@@@@@@@@@@=  .@████████████|
//  9 |████████████@:  =@@@@@@@@*::=@@@@@@@@@@@@@@=  .@████████████|
// 10 |████████████@:  =@@@@@@@@-   @@@@@@@@@@@@@@=  .@████████████|
// 11 |████████████@:  =@@@@@@@@@%%@@@@+:*@@@@@@@@=  .@████████████|
// 12 |████████████@:  =@@@@@@@=+@@@@*.   :%@@@@@@=  .@████████████|
// 13 |████████████@:  =@@@@@+   :*#:       =@@@@@=  .@████████████|
// 14 |████████████@:  =@@@*                  *@@@=  .@████████████|
// 15 |████████████@:  =@@@%##################@@@@=  .@████████████|
// 16 |████████████@+   +%@@@@@@@@@@@@@@@@@@@@@@%+   =@████████████|
// 17 |████████████@@%*++++++++++++++++++++++++++++*%@@████████████|
// 18 |                                                            |
// 19 |> C o n f i r m   S e t t i n g s <                         |
//    +------------------------------------------------------------+
//
macro_rules! rFillText { // Same as python's ljust with pad as " " and length as 30
                         // Make the string 30 chars long, fill with spaces
                         // Wipes background to prevent overwriting
    ($text: expr) => {
        format!("{: <30}", $text)
    };
}
macro_rules! fmtOption {// This macro also pads but handles the "name: <selection>" format
                        // also avoids repeated inline if statement clutter
    ($name: expr, $value: expr, $selected: expr) => {
        if $selected {
            format!("{: <30}", format!("{}: <{}>", $name, $value))
        } else {
            format!("{: <30}", format!("{}: {}", $name, $value))
        }
    };
}
fn split_text(text: impl Into<String>) -> Vec<String> { // Break on whitespace rather than mid-word
    let mut output: Vec<String> = vec![];
    let text: String = text.into();
    let mut iter: Vec<&str> = text.split_ascii_whitespace().collect::<Vec<&str>>(); // Split on spaces

    let mut tmp = "".to_string();
    loop {
        if iter.len() > 0 {
            if tmp.len() + iter[0].len() < 30 { // Append each word with a trailing space until it would be too long to print. Using <30
                                                //  means spaces do not matter at the end since the space would make it 30, so those are unaccounted for here
                                                //  after we reach length just push the string to the end of the output, padded to 30 to clear any underlying text.
                tmp.push_str(iter.remove(0));
                tmp.push_str(" ");
            } else {
                output.push(rFillText!(tmp));
                tmp = "".to_string();
            }
        } else {
            break
        }
    }

    while output.len() < 12 { // Just fill out all 12 lines with text since this is only used here
        output.push(" ".repeat(30));
    }

    output
}
fn render_menu(data: &RenderConfig, selection: &MenuSelection, bitmap: &mut Bitmap3, rewrite_info: bool, mixer: &mut Mixer) {
    // Render the basic menuing stuff
    PIXELARA.print_str_rel(rFillText!("GBA-RT Configuration"), bitmap, 0, 0);
    PIXELARA.print_str_rel(fmtOption!("Scene", get_scene_name(data.scene), *selection == MenuSelection::SceneSelect), bitmap, 0, 2);
    PIXELARA.print_str_rel(fmtOption!("Iters", data.iters_per_pixel, *selection == MenuSelection::IterationsSelect), bitmap, 0, 3);
    PIXELARA.print_str_rel(fmtOption!("Depth", data.max_depth, *selection == MenuSelection::DepthSelect), bitmap, 0, 4);
    PIXELARA.print_str_rel(rFillText!(if *selection == MenuSelection::ConfirmButton {
        "> Confirm Settings <"
    } else {
        "Confirm Settings"
    }), bitmap, 0, 19);

    // Rewrite_info is about 70% of the text rendered and by far the longest to render since we use bitmap mode, so it is rendered only if 100% needed (if you change your menu selection)
    if rewrite_info {
        match selection {
            MenuSelection::SceneSelect => {
                mixer.frame();
                for x in 0*8..6*8 { // Clear left padding of image
                    for y in 6*8..17*8 {
                        bitmap.draw_point(x, y, 0);
                    }
                }
                mixer.frame();
                IMAGES.print_nth(3, bitmap, 6*8, 6*8); // Print image
                mixer.frame();
                for x in 24*8..29*8 { // Clear right padding of image
                    for y in 6*8..17*8 {
                        bitmap.draw_point(x, y, 0);
                    }
                }
                mixer.frame();
            },
            MenuSelection::IterationsSelect => {
                mixer.frame();
                for (i, s) in split_text("How many iterations to run per pixel, more iterations improves aliasing at the cost of performance.").iter().enumerate() { // Print each line of the help message, updating the music between each print
                    mixer.frame();
                    PIXELARA.print_str_rel(s, bitmap, 0, 6 + i);
                    mixer.frame();
                }
            },
            MenuSelection::DepthSelect => {
                mixer.frame();
                for (i, s) in split_text("Max bounces per sample, more bounces will increase accuracy at the cost of performance with diminishing returns.").iter().enumerate() { // Print each line of the help message, updating the music between each print
                    mixer.frame();
                    PIXELARA.print_str_rel(s, bitmap, 0, 6 + i);
                    mixer.frame();
                }
            },
            MenuSelection::ConfirmButton => {
                mixer.frame();
                for x in 0..240 { // No help message, clear the text area
                    for y in 6*8..17*8 {
                        bitmap.draw_point(x, y, 0);
                    }
                }
            },
        }
    }
}

// Spawn gui for render config fetching.
pub fn get_render_config(input: &mut ButtonController, bitmap: &mut Bitmap3, mixer: &mut Mixer) -> RenderConfig {
    let vblank = agb::interrupt::VBlank::get();

    let mut data = RenderConfig {
        iters_per_pixel: 4,
        scene: Scenes::from_i8(0),
        max_depth: 8
    };

    let mut menu_selection = MenuSelection::SceneSelect;

    bitmap.clear(0); // Initial screen clear and populate, force render help message since it would not otherwise
    render_menu(&data, &menu_selection, bitmap, true, mixer);

    loop {
        let mut one_true = true;          // If any button was pressed
        let mut change_menu_text = false; // If that button was up or down, thus requiring a rewrite of the help message.
        mixer.frame();
        if input.is_just_pressed(Button::UP) { // Moves to the previous config option
            menu_selection = menu_selection.prev();
            change_menu_text = true;
        } else if input.is_just_pressed(Button::DOWN) { // Moves to the next config potion
            menu_selection = menu_selection.next();
            change_menu_text = true;
        } else if input.is_just_pressed(Button::RIGHT) { // Will select the next value on all but the confirm button, where it tells the system no button was pressed as nothing actually changed
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
                MenuSelection::ConfirmButton => {
                    one_true = false;
                }
            }
        } else if input.is_just_pressed(Button::LEFT) { // Will select the previous value on all but the confirm button, where it tells the system no button was pressed as nothing actually changed
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
                MenuSelection::ConfirmButton => {
                    one_true = false;
                }
            }
        } else if input.is_just_pressed(Button::A) { // If on the confirm button will leave the loop, otherwise tells the system no button was pressed as nothing actually happened
            if menu_selection == MenuSelection::ConfirmButton {
                break
            } else {
                one_true = false;
            }
        } else {
            one_true = false;
        }

        if one_true { // If a button was pressesd render the menu
            render_menu(&data, &menu_selection, bitmap, change_menu_text, mixer);
        }

        mixer.frame();
        agb::interrupt::VBlank::wait_for_vblank(&vblank); // Wait for vblank, not really needed but good practice
        input.update(); // Update pressed buttons
    }

    return data;
}
