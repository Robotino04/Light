use ultraviolet::Vec3;

use crate::material::Material;

pub struct HitResult{
    pub t: f32,
    pub normal: Vec3,
    pub material: Option<Material>,
}

impl Default for HitResult{
    fn default() -> HitResult{
        HitResult{
            t: f32::INFINITY,
            normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
            material: None,
        }
    }
}
