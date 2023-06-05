use image::Image;

mod image;

fn main() {
    let mut image: Image = Image::new(400, 225);
    
    for x in 0..image.width(){
        for y in 0..image.height(){
            image[(x, y)].r = x as f32 / image.width() as f32;
            image[(x, y)].g = y as f32 / image.height() as f32;
        }   
    }
    
    image.save_to_file("/tmp/test.ppm").unwrap();
}
