use std::{time::Instant, io::{stdout, Write}, sync::{Mutex, mpsc}, thread};
use light::{mesh::Mesh, image::Image, hittable::Hittable, sphere::Sphere, material::Material, camera::Camera, trace_ray};
use rayon::prelude::*;
use ultraviolet::{self, Mat4, Vec3};

enum ProgressMessage{
    AddScanline,
    Quit,
}

fn main() {
    let mut cube = Mesh::from_obj("meshes/default_cube.obj").unwrap();
    cube.apply_matrix(Mat4::from_translation(Vec3::new(1.0, 0.0, -1.0)) * Mat4::from_scale(0.9));

    let protected_image: Mutex<Image> = Mutex::new(Image::new(1920, 1080));
    let image_width: i32 = protected_image.lock().unwrap().width();
    let image_height: i32 = protected_image.lock().unwrap().height();
    let samples_per_pixel: usize = 1000;
    let max_depth: i32 = 50;

    let scene: Vec<Box<dyn Hittable + Sync + Send>> = vec![
        Box::new(Sphere{
            center: Vec3::new(-1.0,0.0,-1.0),
            radius: 0.5,
            material:
                Material::DielectricMaterial{
                    albedo: Vec3::new(1.0,1.0,1.0),
                    ior: 1.5, 
                }
        }),
        Box::new(Sphere{
            center: Vec3::new(-1.0,0.0,-1.0),
            radius: -0.4,
            material:
                Material::DielectricMaterial{
                    albedo: Vec3::new(1.0,1.0,1.0),
                    ior: 1.5, 
                }
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0,1.0,-3.0),
            radius: 0.5,
            material: Material::EmissiveMaterial{
                emission_color: Vec3::new(0.8, 0.3, 0.2),
                strength: 15.0,
            }
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0,0.0,-1.0),
            radius: 0.5,
            material: Material::DiffuseMaterial{
                albedo: Vec3::new(0.1, 0.2, 0.5),
            }
        }),
        Box::new(Mesh{
            triangles: cube.triangles,
            material: Material::MetallicMaterial{
                albedo: Vec3::new(0.8,0.6,0.2),
                roughness: 0.0,
            }
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0,-100.5,0.0),
            radius: 100.0,
            material: Material::DiffuseMaterial{
                albedo: Vec3::new(0.8, 0.8, 0.0),
            },
        }),
    ];

    // setup rendering data
    let camera_pos = Vec3::new(-3.0, 3.0, 2.0);
    let target_pos = Vec3::new(0.0, 0.0, -1.0);
    let depth_of_field = (camera_pos - target_pos).mag();
    let aperture_size = 0.15;

    let camera = Camera::new(camera_pos, target_pos, 35.0, image_width as f32 / image_height as f32, aperture_size, depth_of_field);
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
   
                let u = (x as f32 + x_offset) / image_width as f32;
                let v = (*y as f32 + y_offset) / image_height as f32;

                let ray = camera.get_ray(u, v);

                local_pixel += trace_ray::trace_ray(ray, &scene, max_depth);
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
