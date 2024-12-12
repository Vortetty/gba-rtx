pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod debug_front;

use agb::hash_map::HashMap;
use alloc::boxed::Box;

use crate::{material::Material, color::Color};
use self::{lambertian::LambertianMat, debug_front::DebugFrontMat};

pub struct MatManager {
    pub mat_map: HashMap<i32, Box<dyn Material>>,
    pub mat_counter: i32
}

impl MatManager {
    pub fn new() -> MatManager {
        let mut tmp = MatManager { mat_map: HashMap::new(), mat_counter: -1 };
        tmp.gen_mat(Box::new(DebugFrontMat {}));
        return tmp;
    }

    pub fn gen_mat(&mut self, mat: Box<dyn Material>) -> i32 {
        self.mat_counter += 1;
        self.mat_map.insert(self.mat_counter, mat);
        return self.mat_counter;
    }

    pub fn get_mat(&self, mat: &i32) -> &Box<dyn Material> {
        return &self.mat_map[mat];
    }
}
