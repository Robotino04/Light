use std::{ops::{Index, IndexMut}, path::Path, fs::File, io::Write, slice};

use ultraviolet::Vec3;

#[derive(Debug, Clone)]
pub struct Image{
    pixels: Vec<Vec3>,
    width: i32,
    height: i32,
}

impl Image{
    pub fn new(width: i32, height:i32) -> Image{
        return Image{
            pixels: vec![Vec3{x: 0.0, y: 0.0, z: 0.0}; (width*height) as usize],
            width,
            height,
        };
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()>{
        let path = Path::new(filename);
        let mut file = File::create(& path)?;
        write!(file, "P6\n{} {} 255\n", self.width, self.height)?;

        for y in (0..self.height).rev(){
            for x in 0..self.width{
                let mut pixel_mut: Vec3 = self[(x, y)];
                pixel_mut.clamp(Vec3::new(0.0, 0.0, 0.0), Vec3 { x: 1.0, y: 1.0, z: 1.0});
                // gamma correct for gamma=2
                pixel_mut.x = pixel_mut.x.sqrt();
                pixel_mut.y = pixel_mut.y.sqrt();
                pixel_mut.z = pixel_mut.z.sqrt();
                
                pixel_mut *= 255.0;
                file.write(slice::from_ref(&(pixel_mut.x as u8)))?;
                file.write(slice::from_ref(&(pixel_mut.y as u8)))?;
                file.write(slice::from_ref(&(pixel_mut.z as u8)))?;
            }
        }

        return Ok(());
    }

    pub fn width(&self) -> i32 {self.width}
    pub fn height(&self) -> i32 {self.height}
}

impl Index<(i32, i32)> for Image{
    type Output = Vec3;
    fn index(&self, idx: (i32, i32)) -> &Vec3{
        return self.pixels.index((idx.0 + idx.1 * self.width) as usize);
    }
}

impl IndexMut<(i32, i32)> for Image{
    fn index_mut(&mut self, idx: (i32, i32)) -> &mut Vec3{
        return self.pixels.index_mut((idx.0 + idx.1 * self.width) as usize);
    }
}
