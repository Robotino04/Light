use std::{ops::{Index, IndexMut}, path::Path, fs::File, io::Write, slice};

#[derive(Debug, Clone, Copy)]
pub struct Pixel{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub struct Image{
    pub pixels: Vec<Pixel>,
    width: i32,
    height: i32,
}

impl Image{
    pub fn new(width: i32, height:i32) -> Image{
        return Image{
            pixels: vec![Pixel{r: 0.0, g: 0.0, b: 0.0}; (width*height) as usize],
            width: width,
            height: height,
        };
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()>{
        let path = Path::new(filename);
        let mut file = File::create(& path)?;
        write!(file, "P6\n{} {} 255\n", self.width, self.height)?;

        for pixel in self.pixels.iter(){
            file.write(slice::from_ref(&((pixel.r * 255.0).clamp(0.0, 255.0) as u8)))?;
            file.write(slice::from_ref(&((pixel.g * 255.0).clamp(0.0, 255.0) as u8)))?;
            file.write(slice::from_ref(&((pixel.b * 255.0).clamp(0.0, 255.0) as u8)))?;
        }

        return Ok(());
    }

    pub fn width(&self) -> i32 {self.width}
    pub fn height(&self) -> i32 {self.height}
}

impl Index<(i32, i32)> for Image{
    type Output = Pixel;
    fn index(&self, idx: (i32, i32)) -> &Pixel{
        return self.pixels.index((idx.0 + idx.1 * self.width) as usize);
    }
}

impl IndexMut<(i32, i32)> for Image{
    fn index_mut(&mut self, idx: (i32, i32)) -> &mut Pixel{
        return self.pixels.index_mut((idx.0 + idx.1 * self.width) as usize);
    }
}