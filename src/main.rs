use std::{time::Instant, io::{stdout, Write}, ops::{Mul, Add, AddAssign}, sync::Mutex, thread::{Thread, self}};

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

use rayon::prelude::*;

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
    let image_width: i32 = 400;
    let image_height: i32 = 225;
    let protected_image: Mutex<Image> = Mutex::new(Image::new(image_width, image_height));
    let samples_per_pixel: usize = 100;

    let scene: Vec<Box<dyn Hittable + Sync>> = vec![
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

    // setup rendering data

    let aspect_ratio = image_width as f32 / image_height as f32;
    let camera_matrix = ultraviolet::projection::perspective_gl((90.0 as f32).to_radians(), aspect_ratio, 0.0001, 10000.0);
    let inverse_camera_matrix = camera_matrix.inversed();
    let scanlines = (0..image_height).collect::<Vec<i32>>();
    let progress: Mutex<usize> = Mutex::new(0);

    println!("Rendering {}x{} image @ {} spp...", image_width, image_height, samples_per_pixel);
    
    let progress_bar_thread = thread::spawn(|| {
        while progress.lock().unwrap() < image_height{
        }
    });
    progress_bar_thread.

    let rendering_start = Instant::now();
    scanlines.par_iter().for_each(|y: &i32|{ 
        let mut rng = rand::thread_rng();
        for x in 0..image_width{
            let mut local_pixel: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _sample in 0..samples_per_pixel{
                let x_offset: f32 = rng.gen(); 
                let y_offset: f32 = rng.gen(); 

                let ray: Ray = Ray{
                    direction: (inverse_camera_matrix * Vec4::new(
                        ((x as f32 + x_offset) / image_width as f32) * 2.0 - 1.0,
                        ((*y as f32 + y_offset) / image_height as f32) * 2.0 - 1.0,
                        1.0,
                        1.0
                    )).xyz(), 

                    origin: Vec3::new(0.0,0.0,0.0)
                };

                local_pixel += trace_ray(ray, &scene);
            }
            protected_image.lock().unwrap()[(x, *y)] = local_pixel / samples_per_pixel as f32;
        }
        progress.lock().unwrap().add_assign(1);
    });
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    protected_image.lock().unwrap().save_to_file("/tmp/test.ppm").unwrap();
}
