use ultraviolet::Vec3;

use crate::{hittable::Hittable, ray::Ray, hit_result::HitResult, material::Material};

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool{
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let half_b =  oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // try the first intersection
        let mut t = (-half_b - sqrt_discriminant) / a;
        if t < min_distance || t > hit.t {
            // try the second intersection
            t = (-half_b + sqrt_discriminant) / a;
            if t < min_distance || t > hit.t {
                return false;
            }
        }

        hit.t = t;
        hit.material = Some(self.material);
        hit.set_face_normal(ray.direction, (ray.at(t) - self.center) / self.radius);

        return true;
    }
    fn get_min_bounds(&self) -> Vec3 {
        return self.center - Vec3::new(self.radius, self.radius, self.radius);
    }
    fn get_max_bounds(&self) -> Vec3 {
        return self.center + Vec3::new(self.radius, self.radius, self.radius);
    }
}   

