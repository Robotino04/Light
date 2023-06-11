use ultraviolet::Vec3;

use crate::material::Material;

pub struct HitResult{
    pub t: f32,
    pub normal: Vec3,
    pub material: Option<Material>,
    pub is_front_face: bool,
}

impl HitResult{
    pub fn set_face_normal(&mut self, ray_direction: Vec3, outward_normal: Vec3){
        self.is_front_face = ray_direction.dot(outward_normal) < 0.0;
        self.normal = if self.is_front_face {outward_normal} else {-outward_normal};
    }
}

impl Default for HitResult{
    fn default() -> HitResult{
        HitResult{
            t: f32::INFINITY,
            normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
            material: None,
            is_front_face: false,
        }
    }
}

