use ultraviolet::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray{
    pub fn at(&self, t: f32) -> Vec3{
        self.origin + t * self.direction
    }
}
