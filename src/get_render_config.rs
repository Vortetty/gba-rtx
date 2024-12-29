use core::mem;

use agb::{display::{bitmap3::Bitmap3, busy_wait_for_vblank}, input::{Button, ButtonController}};
use alloc::string::{String, ToString};

use crate::nescentricities::NESCENTRICITIES;

#[derive(Clone, Copy)]
pub enum Scenes {
    SPHERES
}

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

static SELECTABLE_SCENES: [Scenes; 1] = [
    Scenes::SPHERES
];

#[repr(i8)]
enum MenuSelection {
    SceneSelect=0,
    IterationsSelect,
    DepthSelect,
    ConfirmButton
}

pub fn get_render_config(input: &mut ButtonController, bitmap: &mut Bitmap3) -> RenderConfig {
    let vblank = agb::interrupt::VBlank::get();
    let scene = 0usize;
    let iters = 4;
    let depth = 8;

    let mut menu_selection = MenuSelection::SceneSelect;

    loop {
        let mut one_true = true;
        agb::interrupt::VBlank::wait_for_vblank(&vblank);
        if input.is_just_pressed(Button::UP) {
            menu_selection = unsafe {
                mem::transmute((menu_selection as u8).wrapping_sub(1) % (mem::variant_count::<MenuSelection>() as u8))
            };
        } else if input.is_just_pressed(Button::DOWN) {
            menu_selection = unsafe {
                mem::transmute((menu_selection as u8).wrapping_add(1) % (mem::variant_count::<MenuSelection>() as u8))
            };
        } else {
            one_true = false;
        }

        if (one_true) {
            NESCENTRICITIES.print_str(match menu_selection {
                MenuSelection::SceneSelect => "Scene Select    ",
                MenuSelection::IterationsSelect => "Iter Select     ",
                MenuSelection::DepthSelect => "Depth Select    ",
                MenuSelection::ConfirmButton => "Confirm Setting",
            }, bitmap, 0, 0);
        }

        input.update();
    }

    return RenderConfig {
        iters_per_pixel: iters,
        scene: SELECTABLE_SCENES[scene],
        max_depth: depth
    }
}
