use std::{time::Instant, io::{stdout, Write}, ops::{Mul, Add}};

use hittable::Hittable;
use image::Image;
use ray::Ray;

mod image;
mod ray;
mod sphere;
mod hittable;

use ultraviolet::{Vec3, Vec4};

use crate::sphere::Sphere;

fn lerp<T>(t: f32, x0: T, x1: T) -> T
    where T: Add<T, Output = T> + Mul<f32, Output = T>{
        x0*(1.0-t) + x1*t
    }

fn sample_background_gradient(ray: Ray) -> Vec3{
    let t: f32 = 0.5*(ray.direction.y + 1.0);
    return lerp::<Vec3>(t, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0));
}

fn trace_ray(ray: Ray, scene: &impl Hittable) -> Vec3{
    if scene.hit(ray){
        return Vec3::new(1.0, 0.0, 0.0);
    }
    sample_background_gradient(ray)
}

fn main() {
    let mut image: Image = Image::new(400, 225);

    let scene: Sphere = Sphere{
        center: Vec3::new(0.0,0.0,-1.0),
        radius: 0.5,
    };
        

    println!("Rendering {}x{} image...", image.width(), image.height());
    let rendering_start = Instant::now();

    let aspect_ratio = image.width() as f32 / image.height() as f32;
    let camera_matrix = ultraviolet::projection::perspective_gl((90.0 as f32).to_radians(), aspect_ratio, 0.0001, 10000.0);
    let inverse_camera_matrix = camera_matrix.inversed();
    for x in 0..image.width(){
        for y in 0..image.height(){

            let ray: Ray = Ray{
                direction: (inverse_camera_matrix * Vec4::new(
                    (x as f32 / image.width() as f32) * 2.0 - 1.0,
                    (y as f32 / image.height() as f32) * 2.0 - 1.0,
                    1.0,
                    1.0
                )).xyz(), 

                origin: Vec3::new(0.0,0.0,0.0)
            };

            image[(x, y)] = trace_ray(ray, &scene);
        }
        print!("[{:<50}]\r", "#".repeat((x*50 / (image.width()-1)) as usize));
        stdout().flush().unwrap();
    }
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());
    
    image.save_to_file("/tmp/test.ppm").unwrap();
}