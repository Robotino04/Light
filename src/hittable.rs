use crate::{ray::Ray, hit_result::HitResult};

pub trait Hittable{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool;
}

impl Hittable for Vec<Box<dyn Hittable + Sync + Send>>{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool{
        let mut did_hit = false;
        for hittable in self.iter() {
            did_hit |= hittable.hit(ray, hit, min_distance);
        }
        return did_hit;
    }
}
