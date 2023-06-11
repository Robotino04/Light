use ultraviolet::Vec3;

use crate::ray::Ray;

pub struct Camera{
    pos: Vec3,
    lower_left_corner_world_space: Vec3,
    width_world_space: Vec3,
    height_world_space: Vec3,
}

impl Camera{
    pub fn new(fov: f32, aspect_ratio: f32, pos: Vec3, target: Vec3) -> Camera{
        let half_height = (fov.to_radians()/2.0).tan();
        let image_height = half_height * 2.0;
        let image_width = aspect_ratio * image_height;

        let up = Vec3::new(0.0, 1.0, 0.0);
        
        let w = (pos - target).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let width_world_space = image_width * u;
        let height_world_space = image_height * v;


        Camera{
            pos,
            width_world_space,
            height_world_space,
            lower_left_corner_world_space: pos - width_world_space/2.0 - height_world_space/2.0 - w,
        }
    }
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray{
            direction: (self.lower_left_corner_world_space + x*self.width_world_space + y*self.height_world_space - self.pos).normalized(),
            origin: self.pos, 
        }
    }
}

