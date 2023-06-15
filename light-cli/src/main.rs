use std::{time::Instant, io::{stdout, Write}, sync::{Mutex, Arc}, thread};
use light::{mesh::Mesh, image::Image, hittable::Hittable, sphere::Sphere, material::Material, camera::Camera, trace_ray};
use rayon::prelude::*;
use ultraviolet::{self, Mat4, Vec3};

fn main() {
    let mut cube = Mesh::from_obj("meshes/default_cube.obj").unwrap();
    cube.apply_matrix(Mat4::from_translation(Vec3::new(1.0, 0.0, -1.0)) * Mat4::from_scale(0.9));

    let protected_image: Arc<Mutex<Image>> = Arc::new(Mutex::new(Image::new(1920, 1080)));
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

    println!("Rendering {}x{} image @ {} spp; depth {}...", image_width, image_height, samples_per_pixel, max_depth);
    
    // limit to one thread
    //rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    
    let mut saving_thread: Option<thread::JoinHandle<()>> = None;
    let scanlines = (0..image_height).collect::<Vec<i32>>();
    let rendering_start = Instant::now();
    for sample in 0..samples_per_pixel{
        print!("\r[{:>width$}/{:>width$}][{:>6.2}%]{} Rendering...{:>10}",
                sample+1, samples_per_pixel,
                (sample+1) as f32 / samples_per_pixel as f32 * 100.0,
                match saving_thread {Some(_) => "[Saving]", None => ""},
                "",
                width = samples_per_pixel.to_string().len()
        );
        stdout().flush().unwrap();
        scanlines.par_iter().for_each_with(protected_image.clone(), |protected_image, y: &i32|{ 
            let mut this_row: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); image_width as usize];
            for x in 0..image_width{
                let x_offset: f32 = rand::random(); 
                let y_offset: f32 = rand::random(); 

                let u = (x as f32 + x_offset) / image_width as f32;
                let v = (*y as f32 + y_offset) / image_height as f32;

                let ray = camera.get_ray(u, v);

                this_row[x as usize] += trace_ray::trace_ray(ray, &scene, max_depth);
            }
            let mut image = protected_image.lock().unwrap();
            for x in 0..image_width{
                image[(x, *y)] += this_row[x as usize];
            }
        });
        if sample % 40 == 0{
            let mut image_copy = protected_image.lock().unwrap().clone();
            saving_thread = Some(thread::spawn(move || {
                // average all samples
                for x in 0..image_width{
                    for y in 0..image_height{
                        image_copy[(x, y)] /= (sample+1) as f32;
                    }
                }
                image_copy.save_to_file("/tmp/test.ppm").unwrap();
            }));
        }
        match &saving_thread{
            Some(thread) => {
                if thread.is_finished(){
                    saving_thread = None;
                }
            },
            None => {}, 
        };
    }
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    // average all samples
    let mut image = protected_image.lock().unwrap();
    for x in 0..image_width{
        for y in 0..image_height{
            image[(x, y)] /= samples_per_pixel as f32;
        }
    }
    image.save_to_file("/tmp/test.ppm").unwrap();
}
