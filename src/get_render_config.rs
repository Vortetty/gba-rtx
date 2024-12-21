use agb::{display::object::Graphics, include_aseprite};
use alloc::{format,string::{String, ToString}, vec, vec::Vec};

use crate::text::SpriteFont;

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
        _ => "Unimplemented"
    }.to_string()
}

static SELECTABLE_SCENES: [Scenes; 1] = [
    Scenes::SPHERES
];

static simpleFont: &'static Graphics = include_aseprite!("resources/text.aseprite");

pub fn get_render_config(mut gba: agb::Gba) -> RenderConfig {
    let scene = SELECTABLE_SCENES[0];
    let iters = 4;
    let depth = 8;

    let fnt = SpriteFont::load_font(simpleFont);

    return RenderConfig {
        iters_per_pixel: iters,
        scene: scene,
        max_depth: depth
    }
}