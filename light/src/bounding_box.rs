use std::{mem::swap, cmp::Ordering, rc::Rc, ops::Deref, vec};

use ultraviolet::Vec3;

use crate::hittable::Hittable;

pub struct BoundingBox{
    pub min: Vec3,
    pub max: Vec3,
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl BoundingBox{
    pub fn build_bvh(initial_objects: Vec<Box<dyn Hittable + Sync + Send>>) -> Box<dyn Hittable + Sync + Send>{
        let mut working_objects: Vec<Rc<dyn Hittable + Sync + Send>> = Vec::new();
        for x in initial_objects{
            working_objects.push(Rc::from(x));
        }
        while working_objects.len() <= 1{
            let axis = rand::random::<usize>() % 3;
            working_objects.sort_by(|a, b| {
                let a_center = (a.get_min_bounds() + a.get_max_bounds());
                let b_center = (b.get_min_bounds() + b.get_max_bounds());

                if (a_center.x == b_center.x)
                return a_center.total_cmp(&b_center);
            });
            working_objects = working_objects.chunks(2).map(|chunk|{
                if chunk.len() == 2{
                    Box::new(BoundingBox{
                        min: Vec3::zero(),
                        max: Vec3::zero(),
                        objects: vec![chunk[0], chunk[1]],
                    })
                }
                else{
                    chunk[0]
                }
            }).collect();
        }

        return *working_objects.iter().next().unwrap();
    }
}

impl Hittable for BoundingBox{
    fn hit(&self, ray: crate::ray::Ray, hit: &mut crate::hit_result::HitResult, mut min_distance: f32) -> bool {
        for a in 0..3 {
            let inverse_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inverse_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inverse_d;
            if inverse_d < 0.0{
                swap(&mut t0, &mut t1);
            }
            min_distance = t0.max(min_distance);
            hit.t = t1.max(hit.t);
            if hit.t <= min_distance {
                return false;
            }
        }
        return self.objects.hit(ray, hit, min_distance);
    }

    fn get_max_bounds(&self) -> Vec3 {
        self.objects.get_min_bounds()
    }
    fn get_min_bounds(&self) -> Vec3 {
        self.objects.get_max_bounds()
    }
}
