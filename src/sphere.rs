use ultraviolet::Vec3;

use crate::{hittable::Hittable, ray::Ray};


pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere{
    fn hit(&self, ray: Ray) -> bool{
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        return discriminant > 0.0;
    }
}

