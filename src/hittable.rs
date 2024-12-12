use agb::timer::Timer;
use alloc::{boxed::Box, vec::Vec};


use crate::{vec3::Vec3, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub trace_len: f32,
    pub front_face: bool,
    pub material: i32,
    pub tex_u: f32,
    pub tex_v: f32
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot_prod(*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
    pub fn default() -> HitRecord {
        HitRecord {
            point: Vec3 {
                x: (0.0),
                y: (0.0),
                z: (0.0),
            },
            normal: Vec3 {
                x: (0.0),
                y: (0.0),
                z: (0.0),
            },
            trace_len: (0.0),
            front_face: false,
            material: 0,
            tex_u: 0.0,
            tex_v: 0.0
        }
    }
}

pub trait Hittable {
    fn hit(&self, timer: &Timer, ray: &Ray, trace_min_len: f32, trace_max_len: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objs: Vec<Box<dyn Hittable + Sync + Send>>
}
impl HittableList {
    pub fn hit(
        &self,
        timer: &Timer,
        ray: &Ray,
        trace_len_min: f32,
        trace_len_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = trace_len_max;
        let mut tmp_rec: HitRecord = HitRecord::default();

        for obj in &self.objs {
            //let mut bounds = AABB { min: Vec3::newi(0,0,0), max: Vec3::newi(0,0,0) };
            //obj.bounds(&mut bounds);
            //if bounds.hit(ray, trace_len_min,trace_len_max) {
                if obj.hit(timer, ray, trace_len_min, closest_so_far, &mut tmp_rec) {
                    hit_anything = true;
                    closest_so_far = tmp_rec.trace_len;
                    *rec = tmp_rec.clone();
                };
            //};
        }

        return hit_anything;
    }

    //pub fn bounds(&self, output: &mut AABB) -> bool {
    //    let mut tmp = AABB { min: Vec3::newi(0,0,0), max: Vec3::newi(0,0,0) };
    //    let mut fbox = true;
//
    //    for obj in &self.objs {
    //        if obj.bounds(&mut tmp) == false { return false; };
//
    //        if fbox {
    //            output.max = tmp.max;
    //            output.min = tmp.min;
    //        } else {
    //            let tmp1 = self.surrounding_box(&tmp, &output);
    //            output.max = tmp1.max;
    //            output.min = tmp1.min;
    //        }
//
    //        fbox = false;
    //    }
//
    //    return true;
    //}
//
    //fn surrounding_box(&self, b1: &AABB, b2: &AABB) -> AABB {
    //    return AABB {
    //        min: Vec3::new(
    //            f32::min(b1.min.x, b2.min.x),
    //            f32::min(b1.min.y, b2.min.y),
    //            f32::min(b1.min.z, b2.min.z)
    //        ),
    //        max: Vec3::new(
    //            f32::max(b1.max.x, b2.max.x),
    //            f32::max(b1.max.y, b2.max.y),
    //            f32::max(b1.max.z, b2.max.z)
    //        )
    //    }
    //}

    pub fn add(&mut self, obj: Box<dyn Hittable + Sync + Send>) {
        self.objs.push(obj);
    }
}