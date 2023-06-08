use std::{time::Instant, io::{stdout, Write}, ops::{Mul, Add}, sync::{Mutex, mpsc}, thread};

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

use rayon::prelude::*;

fn lerp<T>(t: f32, x0: T, x1: T) -> T
    where T: Add<T, Output = T> + Mul<f32, Output = T>{
        x0*(1.0-t) + x1*t
    }

fn sample_background_gradient(ray: Ray) -> Vec3{
    let t: f32 = 0.5*(ray.direction.y + 1.0);
    return lerp::<Vec3>(t, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0));
}

fn random_in_unit_sphere() -> Vec3{
    loop{  
        let v = Vec3{x: rand::random(), y: rand::random(), z: rand::random()} * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if v.mag_sq() < 1.0{
            return v;
        }
    }
}

fn random_on_unit_sphere() -> Vec3{
    random_in_unit_sphere().normalized()
}

fn trace_ray(ray: Ray, scene: &impl Hittable, depth: i32) -> Vec3{
    if depth == 0{
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let mut hit: HitResult = HitResult::default();
    
    scene.hit(ray, &mut hit);

    return match hit.material {
        Some(mat) => {
            match mat {
                material::Material::NormalMaterial() => hit.normal*0.5 + Vec3::new(0.5, 0.5, 0.5),
                material::Material::DiffuseMaterial { albedo } => {
                    let target = hit.normal + random_on_unit_sphere();
                    let new_ray: Ray = Ray{
                        origin: ray.at(hit.t) + hit.normal * 0e-5,
                        direction: target.normalized(),
                    };
                    return albedo * trace_ray(new_ray, scene, depth-1)
                }
                material::Material::MetallicMaterial { albedo, roughness } => {
                    let mut direction = ray.direction.reflected(hit.normal) + roughness * random_in_unit_sphere(); 
                    if direction.dot(hit.normal) <= 0.0{
                        // the ray got reflected back into the object
                        return Vec3::new(0.0, 0.0, 0.0);
                    }
                    direction.normalize();
                    let new_ray: Ray = Ray{
                        origin: ray.at(hit.t) + hit.normal * 0e-5,
                        direction,
                    };
                    return albedo * trace_ray(new_ray, scene, depth-1)
                }
             }
        },
        None => sample_background_gradient(ray),
    }
}

enum ProgressMessage{
    AddScanline,
    Quit,
}

fn main() {
    let protected_image: Mutex<Image> = Mutex::new(Image::new(400, 225));
    let image_width: i32 = protected_image.lock().unwrap().width();
    let image_height: i32 = protected_image.lock().unwrap().height();
    let samples_per_pixel: usize = 100;
    let max_depth: i32 = 50;

    let scene: Vec<Box<dyn Hittable + Sync + Send>> = vec![
        Box::new(Sphere{
            center: Vec3::new(-1.0,0.0,-1.0),
            radius: 0.5,
            material: material::Material::MetallicMaterial{
                albedo: Vec3::new(0.8,0.8,0.8),
                roughness: 0.3,
            }
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0,0.0,-1.0),
            radius: 0.5,
            material: material::Material::DiffuseMaterial{
                albedo: Vec3::new(0.7,0.3,0.3),
            }
        }),
        Box::new(Sphere{
            center: Vec3::new(1.0,0.0,-1.0),
            radius: 0.5,
            material: material::Material::MetallicMaterial{
                albedo: Vec3::new(0.8,0.6,0.2),
                roughness: 1.0,
            }
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0,-100.5,0.0),
            radius: 100.0,
            material: material::Material::DiffuseMaterial{
                albedo: Vec3::new(0.8, 0.8, 0.0),
            },
        })
    ];

    // setup rendering data
    let aspect_ratio = image_width as f32 / image_height as f32;
    let camera_matrix = ultraviolet::projection::perspective_gl((90.0 as f32).to_radians(), aspect_ratio, 0.0001, 10000.0);
    let inverse_camera_matrix = camera_matrix.inversed();
    let scanlines = (0..image_height).collect::<Vec<i32>>();

    println!("Rendering {}x{} image @ {} spp; depth {}...", image_width, image_height, samples_per_pixel, max_depth);
    
    // setup parallelism
    
    // limit to one thread
    //rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    
    let (tx_progress, rx_progress) = mpsc::channel::<ProgressMessage>();
    let progress_bar_thread = thread::spawn(move || {
        let mut progress: i32 = 0;
        loop{
            match rx_progress.recv().unwrap_or(ProgressMessage::Quit){
                ProgressMessage::AddScanline => {
                    progress += 1;
                    print!("[{:<50}]\r", "#".repeat((progress*50 / (image_height-1)) as usize));
                    stdout().flush().unwrap();
                },
                ProgressMessage::Quit => break,
            }
        }
    });

    let rendering_start = Instant::now();
    scanlines.par_iter().for_each_with(tx_progress.clone(), |tx_progress, y: &i32|{ 
        for x in 0..image_width{
            let mut local_pixel: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _sample in 0..samples_per_pixel{
                let x_offset: f32 = rand::random(); 
                let y_offset: f32 = rand::random(); 

                let ray: Ray = Ray{
                    direction: (inverse_camera_matrix * Vec4::new(
                        ((x as f32 + x_offset) / image_width as f32) * 2.0 - 1.0,
                        ((*y as f32 + y_offset) / image_height as f32) * 2.0 - 1.0,
                        1.0,
                        1.0
                    )).xyz(), 

                    origin: Vec3::new(0.0,0.0,0.0)
                };

                local_pixel += trace_ray(ray, &scene, max_depth);
            }
            protected_image.lock().unwrap()[(x, *y)] = local_pixel / samples_per_pixel as f32;
        }
        tx_progress.send(ProgressMessage::AddScanline).unwrap();
    });
    tx_progress.send(ProgressMessage::Quit).unwrap();
    progress_bar_thread.join().unwrap();
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    protected_image.lock().unwrap().save_to_file("/tmp/test.ppm").unwrap();
}
