use ultraviolet::Vec3;

use crate::{ray::Ray, hit_result::HitResult};

pub trait Hittable: Sync + Send{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool;

    fn get_min_bounds(&self) -> Vec3;
    fn get_max_bounds(&self) -> Vec3;
}

impl Hittable for Vec<Box<dyn Hittable>>{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool{
        let mut did_hit = false;
        for hittable in self.iter() {
            did_hit |= hittable.hit(ray, hit, min_distance);
        }
        return did_hit;
    }
    
    fn get_min_bounds(&self) -> Vec3{
        self.iter().map(|obj| obj.get_min_bounds()).reduce(|a, b| a.min_by_component(b)).unwrap()
    }
    fn get_max_bounds(&self) -> Vec3{
        self.iter().map(|obj| obj.get_max_bounds()).reduce(|a, b| a.max_by_component(b)).unwrap()
    }
}
