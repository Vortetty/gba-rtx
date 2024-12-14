use agb::display::object::Graphics;
use alloc::{format,string::{String, ToString}};

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

pub fn get_render_config(mut gba: agb::Gba) -> RenderConfig {
    let scene = Scenes::SPHERES;
    let iters = 4;
    let depth = 8;
    
}