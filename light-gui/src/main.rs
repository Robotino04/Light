use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{time::{Duration, Instant}, sync::{Mutex, atomic::{AtomicBool, AtomicUsize}, Arc}, thread, io::{stdout, Write}};
use light::{image::Image, trace_ray, image_filters::{gamma_correct, average_samples}, importing::load_from_blender};
use rayon::prelude::*;
use ultraviolet::{self, Vec3};

fn main() {
    let scene = load_from_blender("/tmp/blender_export.toml").unwrap(); 
   
    let protected_image: Arc<Mutex<Image>> = Arc::new(Mutex::new(Image::new(scene.width, scene.height)));
    let samples_per_pixel: usize = 10000;
    let max_depth: i32 = 10;

    let scanlines = (0..scene.height).collect::<Vec<u32>>();

    println!("Rendering {}x{} image @ {} spp; depth {}...", scene.width, scene.height, samples_per_pixel, max_depth);


    // setup parallelism
    
    // limit to one thread
    //rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
   
    let should_end = Arc::new(AtomicBool::new(false));
    let next_sample: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));

    let display_thread_image = protected_image.clone();
    let display_thread_should_end = should_end.clone();
    let display_thread_next_sample = next_sample.clone();
    let display_thread = thread::spawn(move || {
        // setup SDL
        let window_width = 1600;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
 
        let aspect_ratio = scene.height as f32 / scene.width as f32;
        let scaling_factor = window_width as f32 / scene.width as f32;
        let window = video_subsystem.window("Light rendering", (scaling_factor * scene.width as f32) as u32, (scaling_factor * aspect_ratio * scene.width as f32) as u32)
            .position_centered()
            .allow_highdpi()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(Some(sdl2::pixels::PixelFormatEnum::RGB24), scene.width, scene.height).unwrap();

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
            let mut image_copy = display_thread_image.lock().unwrap().clone();
            
            texture.update(None, image_copy
                           .apply_filter(|x| average_samples(display_thread_next_sample.load(std::sync::atomic::Ordering::Relaxed), x))
                           .apply_filter(|x| gamma_correct(2.0, x))
                           .get_bytes_inverse_y()
                           .as_slice()
                           , (3 * scene.width) as usize).unwrap();

            canvas.copy_ex(&texture, None, None, 0.0, None, false, false).unwrap();

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
    });
    
    let mut saving_thread: Option<thread::JoinHandle<()>> = None;
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
        scanlines.par_iter().for_each_with(protected_image.clone(), |protected_image, y: &u32|{ 
            let mut this_row: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); scene.width as usize];
            for x in 0..scene.width{
                let x_offset: f32 = rand::random(); 
                let y_offset: f32 = rand::random(); 

                let u = (x as f32 + x_offset) / scene.width as f32;
                let v = (*y as f32 + y_offset) / scene.height as f32;

                let ray = scene.camera.get_ray(u, v);

                this_row[x as usize] += trace_ray::trace_ray(ray, &scene, max_depth);
            }
            let mut image = protected_image.lock().unwrap();
            for x in 0..scene.width{
                image[(x, *y)] += this_row[x as usize];
            }
        });
        if should_end.load(std::sync::atomic::Ordering::Relaxed){
            break;
        }
        if sample % 40 == 0{
            let mut image_copy = protected_image.lock().unwrap().clone();
            let samples_until_now = sample + 1;
            saving_thread = Some(thread::spawn(move || {
                image_copy 
                    .apply_filter(|x| average_samples(samples_until_now, x))
                    .apply_filter(|x| gamma_correct(2.0, x))
                    .save_to_file("/tmp/test.ppm").unwrap();
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
        next_sample.store(sample+1, std::sync::atomic::Ordering::Relaxed);
    }
    should_end.store(true, std::sync::atomic::Ordering::Relaxed);
    display_thread.join().unwrap();
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    protected_image.lock().unwrap()
        .apply_filter(|x| average_samples(next_sample.load(std::sync::atomic::Ordering::Relaxed), x))
        .apply_filter(|x| gamma_correct(2.0, x))
        .save_to_file("/tmp/test.ppm").unwrap();
}
