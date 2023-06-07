use std::{time::Instant, io::{stdout, Write}, ops::{Mul, Add}};

use hit_result::HitResult;
use hittable::Hittable;
use image::Image;
use ray::Ray;

mod image;
mod ray;
mod sphere;
mod hittable;
mod hit_result;
mod material;

use ultraviolet::{Vec3, Vec4};

use crate::sphere::Sphere;

use rand::Rng;

fn lerp<T>(t: f32, x0: T, x1: T) -> T
    where T: Add<T, Output = T> + Mul<f32, Output = T>{
        x0*(1.0-t) + x1*t
    }

fn sample_background_gradient(ray: Ray) -> Vec3{
    let t: f32 = 0.5*(ray.direction.y + 1.0);
    return lerp::<Vec3>(t, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0));
}

fn trace_ray(ray: Ray, scene: &impl Hittable) -> Vec3{
    let mut hit: HitResult = HitResult::default();
    
    scene.hit(ray, &mut hit);

    return match hit.material {
        Some(mat) => {
            match mat {
                material::Material::NormalMaterial() => hit.normal*0.5 + Vec3::new(0.5, 0.5, 0.5),
            }
        },
        None => sample_background_gradient(ray),
    }
}

fn main() {
    let mut image: Image = Image::new(400, 225);
    let samples_per_pixel: usize = 100;

    let scene: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere{
            center: Vec3::new(0.0,0.0,-1.0),
            radius: 0.5,
            material: material::Material::NormalMaterial(),
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0,-100.5,0.0),
            radius: 100.0,
            material: material::Material::NormalMaterial(),
        })
    ];

    let mut rng = rand::thread_rng();

    println!("Rendering {}x{} image @ {} spp...", image.width(), image.height(), samples_per_pixel);
    let rendering_start = Instant::now();

    let aspect_ratio = image.width() as f32 / image.height() as f32;
    let camera_matrix = ultraviolet::projection::perspective_gl((90.0 as f32).to_radians(), aspect_ratio, 0.0001, 10000.0);
    let inverse_camera_matrix = camera_matrix.inversed();
    for x in 0..image.width(){
        for y in 0..image.height(){
            for _sample in 0..samples_per_pixel{
                let x_offset: f32 = rng.gen(); 
                let y_offset: f32 = rng.gen(); 

                let ray: Ray = Ray{
                    direction: (inverse_camera_matrix * Vec4::new(
                        ((x as f32 + x_offset) / image.width() as f32) * 2.0 - 1.0,
                        ((y as f32 + y_offset) / image.height() as f32) * 2.0 - 1.0,
                        1.0,
                        1.0
                    )).xyz(), 

                    origin: Vec3::new(0.0,0.0,0.0)
                };

                image[(x, y)] += trace_ray(ray, &scene);
            }
            image[(x, y)] /= samples_per_pixel as f32;
        }
        print!("[{:<50}]\r", "#".repeat((x*50 / (image.width()-1)) as usize));
        stdout().flush().unwrap();
    }
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    image.save_to_file("/tmp/test.ppm").unwrap();
}
