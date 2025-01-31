use alloc::vec::Vec;
use arrayvec::ArrayVec;
use lambertian::LambertianMat;
use metal::MetalMat;

use crate::math::{ray::Ray, types::FixFlt, vec3::Vec3};

use super::objects::HitRecord;

#[derive(Clone, Copy, Default)]
pub enum MaterialType {
    #[default]
    LAMBERTIAN,
    METAL
}

#[derive(Clone, Copy, Default)]
pub struct Material {
    pub mat_id: usize,
    pub mat_type: MaterialType,
}

pub struct MaterialManager {
    lambertian_mats: ArrayVec<LambertianMat, 256>,
    metal_mats: ArrayVec<MetalMat, 256>
}

impl MaterialManager {
    pub fn add_lambertian(&mut self, albedo: Vec3) -> Material {
        let id = self.lambertian_mats.len();
        unsafe {
            self.lambertian_mats.push_unchecked(LambertianMat {
                albedo
            });
        }
        Material {
            mat_id: id,
            mat_type: MaterialType::LAMBERTIAN
        }
    }
    pub fn add_metal(&mut self, albedo: Vec3) -> Material {
        let id = self.metal_mats.len();
        unsafe {
            self.metal_mats.push_unchecked(MetalMat {
                albedo
            });
        }
        Material {
            mat_id: id,
            mat_type: MaterialType::METAL
        }
    }

    pub fn scatter(&self, material: &Material, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3) {
        match material.mat_type {
            MaterialType::LAMBERTIAN => unsafe {self.lambertian_mats.get_unchecked(material.mat_id)}.scatter(r, rng, hitrec),
            MaterialType::METAL => unsafe {self.metal_mats.get_unchecked(material.mat_id)}.scatter(r, rng, hitrec),
        }
    }

    pub fn new() -> Self {
        Self {
            lambertian_mats: ArrayVec::<LambertianMat, 256>::new(),
            metal_mats: ArrayVec::<MetalMat, 256>::new(),
        }
    }
}

trait Scatterable {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3);
}

mod lambertian;
mod metal;