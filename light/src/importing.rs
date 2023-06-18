use std::{fs::read_to_string, error::Error};

use ultraviolet::Vec3;

use crate::{parsing_error::ParsingError, mesh::Mesh, scene::Scene, camera::Camera};

enum ObjectHeader{
    Mesh,
    Sphere,
    Camera,
}

macro_rules! handle_keys {
    ($type:ty, $($key_name:ident => $key_block:block),+) => {
   }

}

#[derive(Default)]
struct CameraParameters{
    pub pos: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub aperture_size: f32,
    pub depth_of_field: f32,
}

pub fn load_from_blender(filename: &str) -> Result<Scene, Box<dyn Error>>{
    let mut scene: Scene = Scene::default();

    let file_content = read_to_string(filename)?;

    let mut line_number: usize = 0;
    let mut lines = file_content.lines();
    while let Some(raw_line) = lines.next(){
        let line = raw_line.trim();
        line_number += 1;

        let tmp = line.clone().chars().collect::<Vec<_>>();
        let mut iter = tmp.iter();
        match iter.next(){
            Some('[') => {
                match parse_object_header(iter, filename, line_number)?{ 
                    ObjectHeader::Mesh => {
                        let mut object: Option<Mesh> = None;
                        match lines.next(){
                            Some(line) => {
                                if let [key, value] = &line.split("=").take(2).collect::<Vec<_>>()[..]{
                                    match *key{
                                        "mesh_file" => {
                                            object = Some(Mesh::from_obj(value)?);
                                        },
                                        _ => {
                                            return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: format!("Unimplemented key '{}'.", key)}));
                                        }
                                    }
                                }
                                else{
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: "Hit end of line while parsing key.".to_owned()}));
                                }
                            },
                            None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: "Hit end of file while parsing mesh object.".to_string()})); },
                        };
                        if let Some(obj) = object{
                            scene.objects.push(Box::new(obj));
                        }
                        else{
                            return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: "No mesh file proveded for mesh file object".to_owned()}));
                        }
                    },
                    ObjectHeader::Camera => {
                        let mut cam_params: CameraParameters = CameraParameters::default();
                        match lines.next(){
                            Some(line) => {
                                if let [key, value] = &line.split("=").map(|x| x.trim()).take(2).collect::<Vec<_>>()[..]{
                                    match *key{
                                        "fov" => {
                                            cam_params.fov = value.parse::<f32>()?;
                                        },
                                        _ => {
                                            return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: format!("Unimplemented key '{}'.", key)}));
                                        }
                                    }
                                }
                                else{
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: "Hit end of line while parsing key.".to_owned()}));
                                }
                            },
                            None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: "Hit end of file while parsing mesh object.".to_string()})); },
                        };
                        scene.camera = Camera::new(cam_params.pos, cam_params.target, cam_params.fov, cam_params.aspect_ratio, cam_params.aperture_size, cam_params.depth_of_field); 
                    },
                    ObjectHeader::Sphere =>  {todo!();}
                }

            }
            None => {},
            Some(x) => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: format!("Unknown line beginning with '{}'", x)})); },
        }
    }

    return Ok(scene);
}


fn parse_object_header<'a, I>(mut line: I, filename: &str, line_number: usize) -> Result<ObjectHeader, Box<dyn Error>>
    where
        I: DoubleEndedIterator<Item = &'a char> + Clone{
    let object_type = line.clone().take_while(|x| **x != ']').collect::<String>();
    match line.next_back(){
        Some(']') => {},
        Some(x) => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: format!("Invalid last character '{}' for closing bracket of object header.", x).to_string()})); },
        None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: "Hit end of line while parsing object header.".to_string()})); },
    };
    return match object_type.as_str(){
        "mesh" => Ok(ObjectHeader::Mesh),
        "sphere" => Ok(ObjectHeader::Sphere),
        "camera" => Ok(ObjectHeader::Camera),
        x => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: format!("Unknown object type '{}'", x)})); },
    };
}
