use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{time::{Duration, Instant}, sync::{Mutex, atomic::AtomicBool, Arc}, thread};
use light::{mesh::Mesh, image::Image, hittable::Hittable, sphere::Sphere, material::Material, camera::Camera, trace_ray};
use rayon::prelude::*;
use ultraviolet::{self, Mat4, Vec3};

fn main() {
    let protected_image: Arc<Mutex<Image>> = Arc::new(Mutex::new(Image::new(1920, 1080)));
    let image_width: i32 = protected_image.lock().unwrap().width();
    let image_height: i32 = protected_image.lock().unwrap().height();
    let samples_per_pixel: usize = 1000;
    let max_depth: i32 = 50;

    let mut cube = Mesh::from_obj("meshes/default_cube.obj").unwrap();
    cube.apply_matrix(Mat4::from_translation(Vec3::new(1.0, 0.0, -1.0)) * Mat4::from_scale(0.9));


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
   
    let should_end = Arc::new(AtomicBool::new(false));

    let display_thread_image = protected_image.clone();
    let display_thread_should_end = should_end.clone();
    let display_thread = thread::spawn(move || {
        // setup SDL

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Light rendering", image_width as u32, image_height as u32)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(Some(sdl2::pixels::PixelFormatEnum::RGB24), image_width as u32, image_height as u32).unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();
        while !display_thread_should_end.load(std::sync::atomic::Ordering::Relaxed) {
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            display_thread_should_end.store(true, std::sync::atomic::Ordering::Relaxed);
                        },
                    _ => {}
                }
            }
            texture.update(None, display_thread_image.lock().unwrap().get_bytes_inverse_y().as_slice(), (3 * image_width) as usize).unwrap();

            canvas.copy_ex(&texture, None, None, 0.0, None, false, false).unwrap();

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
    });

    let rendering_start = Instant::now();
    for sample in 0..samples_per_pixel{
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
                image[(x, *y)] = (image[(x, *y)] * sample  as f32 + this_row[x as usize]) / ((sample + 1) as f32 );
            }
        });
    }
    should_end.store(true, std::sync::atomic::Ordering::Relaxed);
    display_thread.join().unwrap();
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    protected_image.lock().unwrap().save_to_file("/tmp/test.ppm").unwrap();
    

    
}