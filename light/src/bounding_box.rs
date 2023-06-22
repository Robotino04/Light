use std::mem::swap;

use ultraviolet::Vec3;

use crate::hittable::Hittable;

struct EmptyHittable{}

impl Hittable for EmptyHittable{
    fn get_max_bounds(&self) -> Vec3 {
        unreachable!()
    }
    fn get_min_bounds(&self) -> Vec3 {
        unreachable!()
    }
    fn hit(&self, _: crate::ray::Ray, _: &mut crate::hit_result::HitResult, _: f32) -> bool {
        unreachable!()
    }
}

pub struct BVH<T: ?Sized>{
    bbox: BoundingBox,
    left: Box<T>,
    right: Box<T>,
}

pub struct BoundingBox{
    pub min: Vec3,
    pub max: Vec3,
}

impl<T: ?Sized> BVH<T>{
    pub fn build_recursive(mut objects: Vec<Box<dyn Hittable>>) -> Box<dyn Hittable>{
        if objects.len() == 1{
            return objects.remove(0);
        }
        else{
            let axis = rand::random::<u32>() % 3;
            objects.sort_unstable_by(|a, b|{
                return a.get_min_bounds()[axis as usize].total_cmp(&b.get_min_bounds()[axis as usize]);
            });

            let right = objects.split_off(objects.len()/2);
            
            let mut bvh = Box::new(BVH::<dyn Hittable>{
                left: BVH::<dyn Hittable>::build_recursive(objects) as Box<dyn Hittable>,
                right: BVH::<dyn Hittable>::build_recursive(right) as Box<dyn Hittable>,
                bbox: BoundingBox { min: Vec3::zero(), max: Vec3::zero() }
            });

            bvh.bbox.min = bvh.left.get_min_bounds().min_by_component(bvh.right.get_min_bounds());
            bvh.bbox.max = bvh.left.get_max_bounds().max_by_component(bvh.right.get_max_bounds());

            println!("Min {:?}", bvh.bbox.min);
            println!("Max {:?}", bvh.bbox.max);


            return bvh;
        }
    }

}

// pub static NUM_INTERSECTIONS_PASSED: AtomicUsize = AtomicUsize::new(0);
impl<T: Hittable + ?Sized> Hittable for BVH<T>{
    fn hit(&self, ray: crate::ray::Ray, hit: &mut crate::hit_result::HitResult, min_distance: f32) -> bool {
        let mut t_min = min_distance;
        let mut t_max = hit.t;
        for a in 0..3 {
            let inverse_direction = 1.0 / ray.direction[a];
            let mut t0 = (self.bbox.min[a] - ray.origin[a]) * inverse_direction;
            let mut t1 = (self.bbox.max[a] - ray.origin[a]) * inverse_direction;
            if inverse_direction < 0.0{
                swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min {t0} else {t_min};
            t_max = if t1 < t_max {t1} else {t_max};
            if t_max <= t_min{
                return false;
            }
        }
        // NUM_INTERSECTIONS_PASSED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return self.left.hit(ray, hit, min_distance) | self.right.hit(ray, hit, min_distance);
    }

    fn get_max_bounds(&self) -> Vec3 {
        self.bbox.max
    }
    fn get_min_bounds(&self) -> Vec3 {
        self.bbox.min
    }
}
