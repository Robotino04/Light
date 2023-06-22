use light::{
    image::Image,
    image_filters::{average_samples, gamma_correct},
    importing::load_from_blender,
    trace_ray,
};
use rayon::prelude::*;
use std::{
    io::{stdout, Write},
    sync::{atomic::AtomicUsize, Arc, Mutex},
    thread,
    time::Instant,
};
use ultraviolet::Vec3;

fn main() {
    let scene = Arc::new(load_from_blender("/tmp/blender_export.toml").unwrap());

    let protected_image: Arc<Mutex<Image>> =
        Arc::new(Mutex::new(Image::new(scene.width, scene.height)));
    let samples_per_pixel: usize = 1000;
    let max_depth: i32 = 50;

    println!(
        "Rendering {}x{} image @ {} spp; depth {}...",
        scene.width, scene.height, samples_per_pixel, max_depth
    );

    // limit to one thread
    rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();

    let mut saving_thread: Option<thread::JoinHandle<()>> = None;
    let scanlines = (0..scene.height).collect::<Vec<u32>>();
    let rendering_start = Instant::now();
    let next_sample: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));

    // let ctrl_c_next_sample = next_sample.clone();
    // let ctrl_c_image = protected_image.clone();
    // ctrlc::set_handler(move || {
    //     ctrl_c_image
    //         .lock()
    //         .unwrap()
    //         .apply_filter(|x| {
    //             average_samples(
    //                 ctrl_c_next_sample.load(std::sync::atomic::Ordering::Relaxed),
    //                 x,
    //             )
    //         })
    //         .apply_filter(|x| gamma_correct(2.0, x))
    //         .save_to_file("/tmp/test.ppm")
    //         .unwrap();
    // })
    // .expect("Error setting Ctrl-C handler");

    if let Some(bvh) = &scene.bvh {
        for sample in 0..samples_per_pixel {
            print!(
                "\r[{:>width$}/{:>width$}][{:>6.2}%]{} Rendering...{:>10}",
                sample + 1,
                samples_per_pixel,
                (sample + 1) as f32 / samples_per_pixel as f32 * 100.0,
                match saving_thread {
                    Some(_) => "[Saving]",
                    None => "",
                },
                "",
                width = samples_per_pixel.to_string().len()
            );
            stdout().flush().unwrap();
            scanlines.par_iter().for_each_with(
                protected_image.clone(),
                |protected_image, y: &u32| {
                    let mut this_row: Vec<Vec3> =
                        vec![Vec3::new(0.0, 0.0, 0.0); scene.width as usize];

                    for x in 0..scene.width {
                        let x_offset: f32 = rand::random();
                        let y_offset: f32 = rand::random();

                        let u = (x as f32 + x_offset) / scene.width as f32;
                        let v = (*y as f32 + y_offset) / scene.height as f32;

                        let ray = scene.camera.get_ray(u, v);
                        this_row[x as usize] += trace_ray::trace_ray(ray, bvh, max_depth);
                    }
                    let mut image = protected_image.lock().unwrap();
                    for x in 0..scene.width {
                        image[(x, *y)] += this_row[x as usize];
                    }
                },
            );
            if sample % 40 == 0 {
                let mut image_copy = protected_image.lock().unwrap().clone();
                let samples_until_now = sample + 1;
                saving_thread = Some(thread::spawn(move || {
                    image_copy
                        .apply_filter(|x| average_samples(samples_until_now, x))
                        .apply_filter(|x| gamma_correct(2.0, x))
                        .save_to_file("/tmp/test.ppm")
                        .unwrap();
                }));
            }
            match &saving_thread {
                Some(thread) => {
                    if thread.is_finished() {
                        saving_thread = None;
                    }
                }
                None => {}
            };
            next_sample.store(sample + 1, std::sync::atomic::Ordering::Relaxed);
        }
    }
    else{
        panic!("Scene doesn't contain a BVH");
    }
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());

    protected_image
        .lock()
        .unwrap()
        .apply_filter(|x| {
            average_samples(next_sample.load(std::sync::atomic::Ordering::Relaxed), x)
        })
        .apply_filter(|x| gamma_correct(2.0, x))
        .save_to_file("/tmp/test.ppm")
        .unwrap();
}
