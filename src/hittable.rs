use crate::ray::Ray;

pub trait Hittable{
    fn hit(&self, ray: Ray) -> bool;
}
