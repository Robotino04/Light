use std::{ops::{Index, IndexMut}, path::Path, fs::File, io::Write};

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

        file.write(self.get_bytes_inverse_y().as_slice())?;
        return Ok(());
    }

    pub fn get_bytes_inverse_y(&self) -> Vec<u8>{
        let mut bytes: Vec<u8> = Vec::new();
       
        for y in (0..self.height).rev() {
            for x in 0..self.width{
                let mut pixel_mut = self[(x, y)];
                pixel_mut.clamp(Vec3::zero(), Vec3::one());
                pixel_mut *= 255.0;
                bytes.push(pixel_mut.x as u8);
                bytes.push(pixel_mut.y as u8);
                bytes.push(pixel_mut.z as u8);
            }
        }

        return bytes; 
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let mut bytes: Vec<u8> = Vec::new();
       
        for y in 0..self.height {
            for x in 0..self.width{
                let mut pixel_mut = self[(x, y)];
                pixel_mut.clamp(Vec3::zero(), Vec3::one());
                pixel_mut *= 255.0;
                bytes.push(pixel_mut.x as u8);
                bytes.push(pixel_mut.y as u8);
                bytes.push(pixel_mut.z as u8);
            }
        }

        return bytes; 
    }

    pub fn apply_filter<F>(&mut self, filter: F) -> &mut Image 
        where F: Fn(Vec3) -> Vec3{
            for y in 0..self.height {
                for x in 0..self.width{
                    self[(x, y)] = filter(self[(x, y)]);
                }
            }
            return self;
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
