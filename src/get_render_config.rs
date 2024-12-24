use agb::{display::{object::{OamUnmanaged, ObjectTextRender, PaletteVram, Size}, palette16::Palette16, Font}, include_font};
use alloc::string::{String, ToString};
use core::fmt::Write;

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

static SIMPLE_FONT: Font = include_font!("resources/font/NEScentricities.ttf", 16);

pub fn get_render_config() -> RenderConfig {
    let scene = SELECTABLE_SCENES[0];
    let iters = 4;
    let depth = 8;

    SIMPLE_FONT.

    return RenderConfig {
        iters_per_pixel: iters,
        scene: scene,
        max_depth: depth
    }
}