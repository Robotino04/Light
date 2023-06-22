use crate::{camera::Camera, hittable::Hittable};

#[derive(Default)]
pub struct Scene{
    pub camera: Camera,
    pub bvh: Option<Box<dyn Hittable>>,
    pub width: u32,
    pub height: u32,
}

impl Hittable for Scene{
    fn hit(&self, ray: crate::ray::Ray, hit: &mut crate::hit_result::HitResult, min_distance: f32) -> bool {
        return self.bvh.as_ref().expect("scene doesn't contain a BVH").hit(ray, hit, min_distance);
    }
    fn get_min_bounds(&self) -> ultraviolet::Vec3 {
        self.bvh.as_ref().expect("scene doesn't contain a BVH").get_min_bounds()
    }
    fn get_max_bounds(&self) -> ultraviolet::Vec3 {
        self.bvh.as_ref().expect("scene doesn't contain a BVH").get_max_bounds()
    }
}
