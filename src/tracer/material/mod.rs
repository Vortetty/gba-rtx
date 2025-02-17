use alloc::vec::Vec;
use dielectric::DielectricMat;
use lambertian::LambertianMat;
use metal::MetalMat;
use tinyvec::ArrayVec;

use crate::math::{ray::Ray, types::FixFlt, vec3::Vec3};

use super::objects::HitRecord;

#[derive(Clone, Copy, Default)]
pub enum MaterialType {
    #[default]
    LAMBERTIAN,
    METAL,
    DIELECTRIC,
}

#[derive(Clone, Copy, Default)]
pub struct Material {
    pub mat_id: usize,
    pub mat_type: MaterialType,
}

pub struct MaterialManager {
    lambertian_mats: ArrayVec<[LambertianMat; 256]>,
    metal_mats: ArrayVec<[MetalMat; 256]>,
    dielectric_mats: ArrayVec<[DielectricMat; 256]>,
}

impl MaterialManager {
    pub fn add_lambertian(&mut self, albedo: Vec3) -> Material {
        let id = self.lambertian_mats.len();
        unsafe {
            self.lambertian_mats.push(LambertianMat { albedo });
        }
        Material {
            mat_id: id,
            mat_type: MaterialType::LAMBERTIAN,
        }
    }
    pub fn add_metal(&mut self, albedo: Vec3, matteness: FixFlt) -> Material {
        let id = self.metal_mats.len();
        unsafe {
            self.metal_mats.push(MetalMat {
                albedo,
                matte: matteness,
            });
        }
        Material {
            mat_id: id,
            mat_type: MaterialType::METAL,
        }
    }
    pub fn add_dielectric(&mut self, albedo: Vec3, refraction_index: FixFlt) -> Material {
        let id = self.metal_mats.len();
        unsafe {
            self.dielectric_mats.push(DielectricMat {
                albedo,
                refraction: refraction_index,
                refraction_recip: refraction_index.recip(),
            });
        }
        Material {
            mat_id: id,
            mat_type: MaterialType::DIELECTRIC,
        }
    }

    pub fn scatter(
        &self,
        material: &Material,
        r: &Ray,
        rng: &mut FixFlt,
        hitrec: &HitRecord,
    ) -> (Ray, Vec3, bool) {
        match material.mat_type {
            MaterialType::LAMBERTIAN => {
                unsafe { self.lambertian_mats.get_unchecked(material.mat_id) }
                    .scatter(r, rng, hitrec)
            }
            MaterialType::METAL => {
                unsafe { self.metal_mats.get_unchecked(material.mat_id) }.scatter(r, rng, hitrec)
            }
            MaterialType::DIELECTRIC => {
                unsafe { self.dielectric_mats.get_unchecked(material.mat_id) }
                    .scatter(r, rng, hitrec)
            }
        }
    }

    pub const fn new() -> Self {
        Self {
            lambertian_mats: ArrayVec::from_array_empty(
                [LambertianMat {
                    albedo: Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one()),
                }; 256],
            ),
            metal_mats: ArrayVec::from_array_empty(
                [MetalMat {
                    albedo: Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one()),
                    matte: FixFlt::zero(),
                }; 256],
            ),
            dielectric_mats: ArrayVec::from_array_empty(
                [DielectricMat {
                    albedo: Vec3::new(FixFlt::one(), FixFlt::one(), FixFlt::one()),
                    refraction: FixFlt::zero(),
                    refraction_recip: FixFlt::zero(),
                }; 256],
            ),
        }
    }
}

trait Scatterable {
    fn scatter(&self, r: &Ray, rng: &mut FixFlt, hitrec: &HitRecord) -> (Ray, Vec3, bool);
}

mod dielectric;
mod lambertian;
mod metal;
