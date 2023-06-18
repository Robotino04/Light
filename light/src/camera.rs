use ultraviolet::Vec3;

use crate::ray::Ray;


fn random_in_unit_disk() -> Vec3{
    loop{  
        let v = Vec3{x: rand::random(), y: rand::random(), z: 0.5} * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if v.mag_sq() < 1.0{
            return v;
        }
    }
}

#[derive(Default)]
pub struct Camera{
    pos: Vec3,
    lower_left_corner_world_space: Vec3,
    width_world_space: Vec3,
    height_world_space: Vec3,
    lens_radius: f32,

    u: Vec3,
    v: Vec3,
}

impl Camera{
    pub fn new(pos: Vec3, target: Vec3, fov: f32, aspect_ratio: f32, aperture_size: f32, depth_of_field: f32) -> Camera{
        let half_height = (fov.to_radians()/2.0).tan();
        let image_height = half_height * 2.0;
        let image_width = aspect_ratio * image_height;

        let up = Vec3::new(0.0, 1.0, 0.0);
        
        let w = (pos - target).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let width_world_space = depth_of_field * image_width * u;
        let height_world_space = depth_of_field * image_height * v;

        
        Camera{
            pos,
            width_world_space,
            height_world_space,
            lower_left_corner_world_space: pos - width_world_space/2.0 - height_world_space/2.0 - depth_of_field * w,
            lens_radius: aperture_size / 2.0,
            u, v,
        }
    }
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        // depth of field
        let point_on_lens = self.lens_radius * random_in_unit_disk();
        let offset = self.u * point_on_lens.x + self.v * point_on_lens.y;
    
        Ray{
            direction: (self.lower_left_corner_world_space + x*self.width_world_space + y*self.height_world_space - self.pos - offset).normalized(),
            origin: self.pos + offset, 
        }
    }
}

