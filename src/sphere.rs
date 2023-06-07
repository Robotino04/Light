use ultraviolet::Vec3;

use crate::{hittable::Hittable, ray::Ray, hit_result::HitResult, material::Material};


pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere{
    fn hit(&self, ray: Ray, hit: &mut HitResult) -> bool{
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let half_b =  oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return false;
        }

        let t = (-half_b - discriminant.sqrt() ) / a;
        if t < hit.t && t >= 0.0{
            hit.t = t;
            hit.normal = (ray.at(t) - self.center).normalized();
            hit.material = Some(self.material);
                
            return true;
        }
        return false;
    }
}

