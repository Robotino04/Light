use std::{time::Instant, io::{stdout, Write}};

use image::Image;

mod image;

fn main() {
    let mut image: Image = Image::new(400, 225);
    
    println!("Rendering {}x{} image...", image.width(), image.height());
    let rendering_start = Instant::now();
    for x in 0..image.width(){
        for y in 0..image.height(){
            image[(x, y)].r = x as f32 / image.width() as f32;
            image[(x, y)].g = y as f32 / image.height() as f32;
        }
        print!("[{:<50}]\r", "#".repeat((x*50 / (image.width()-1)) as usize));
        stdout().flush().unwrap();
    }
    println!();
    println!("Rendering took {:.2?}", rendering_start.elapsed());
    
    image.save_to_file("/tmp/test.ppm").unwrap();
}
