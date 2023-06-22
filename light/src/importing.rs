use std::{fs::read_to_string, error::Error, iter::Peekable};

use ultraviolet::Vec3;

use crate::{parsing_error::ParsingError, mesh::Mesh, scene::Scene, camera::Camera, material::Material, bounding_box::BVH, hittable::Hittable, sphere::Sphere};

enum ObjectHeader{
    Mesh,
    Sphere,
    Camera,
}


#[derive(Default)]
struct CameraParameters{
    pub pos: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub aperture_size: f32,
    pub depth_of_field: f32,
}

pub fn load_from_blender(filename: &str) -> Result<Scene, Box<dyn Error>>{
    let mut scene: Scene = Scene::default();
    let mut objects: Vec<Box<dyn Hittable>> =Vec::new();

    let file_content = read_to_string(filename)?;

    let mut line_number: usize = 0;
    let mut lines = file_content.lines().peekable();
    while let Some(raw_line) = lines.next(){
        let line = raw_line.trim();
        line_number += 1;

        let tmp = line.clone().chars().collect::<Vec<_>>();
        let mut iter = tmp.iter();
        match iter.next(){
            Some('[') => {
                match parse_object_header(iter, filename, line_number)?{ 
                    ObjectHeader::Mesh => {
                        let mut obj = parse_mesh_object(&mut lines, filename, &mut line_number)?;
                        obj.material = parse_material(&mut lines, filename, &mut line_number)?;
                        objects.push(Box::new(obj));
                    },
                    ObjectHeader::Sphere =>  {
                        let mut obj = parse_sphere_object(&mut lines, filename, &mut line_number)?;
                        obj.material = parse_material(&mut lines, filename, &mut line_number)?;
                        objects.push(Box::new(obj));
                    },
                    ObjectHeader::Camera => {
                       (scene.camera, scene.width, scene.height) = parse_camera(&mut lines, filename, &mut line_number)?;
                    }
                }

            }
            None => {},
            Some(x) => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: line_number, message: format!("Unknown line beginning with '{}'", x)})); },
        }
    }

    scene.bvh = Some(BVH::<dyn Hittable>::build_recursive(objects));
    // scene.bvh = Some(Box::new(objects));

    return Ok(scene);
}

fn parse_vec3(value: &str) -> Result<Vec3, Box<dyn Error>> {
    let mut parts = value.split(";").map(|x| x.trim()).map(|x| x.parse::<f32>());
    Ok(Vec3{
        x: parts.next().unwrap()?,
        y: parts.next().unwrap()?,
        z: parts.next().unwrap()?,
    })
} 

fn parse_material<'a, I>(lines: &mut Peekable<I>, filename: &str, line_number: &mut usize) -> Result<Material, Box<dyn Error>>
where
I: DoubleEndedIterator<Item = &'a str> + Clone{
    let mut mat = Material::NormalMaterial();
    while (*lines.peek().or(Some(&"[]")).unwrap()).chars().nth(0) != Some('['){
        match lines.next(){
            Some(line) => {
                if let [key, value] = &line.split("=").map(|x| x.trim()).take(2).collect::<Vec<_>>()[..]{
                    match *key{
                        "material_type" => {
                            mat = match *value{
                                "diffuse_material" => Material::DiffuseMaterial{albedo: Vec3::one()},
                                "metallic_material" => Material::MetallicMaterial{albedo: Vec3::one(), roughness: 0.0},
                                "emissive_material" => Material::EmissiveMaterial{emission_color: Vec3::one(), strength: 0.5},
                                "dielectric_material" => Material::DielectricMaterial{albedo: Vec3::one(), ior: 1.0},
                                _ => {
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Unimplemented material type while parsing material '{}'.", key)}));
                                }
                            };
                        },
                        "albedo" => {
                            match &mut mat{
                                Material::MetallicMaterial { albedo, roughness: _} => {
                                    *albedo = parse_vec3(value)?; 
                                }
                                Material::DiffuseMaterial { albedo } => {
                                    *albedo = parse_vec3(value)?; 
                                }
                                Material::DielectricMaterial { albedo, ior: _ } => {
                                    *albedo = parse_vec3(value)?;
                                }
                                _ => {
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Emissive material has no property '{}'.", key)}));
                                }
                            }
                        }
                         "ior" => {
                            match &mut mat{
                                Material::DielectricMaterial{ albedo: _, ior } => {
                                    *ior = value.parse::<f32>()?; 
                                }
                                _ => {
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Emissive material has no property '{}'.", key)}));
                                }
                            }
                        }
                        "emission_color" => {
                            match &mut mat{
                                Material::EmissiveMaterial { emission_color, strength: _ } => {
                                    *emission_color = parse_vec3(value)?; 
                                }
                                _ => {
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Emissive material has no property '{}'.", key)}));
                                }
                            }
                        }
                        "strength" => {
                            match &mut mat{
                                Material::EmissiveMaterial { emission_color: _, strength } => {
                                    *strength = value.parse::<f32>()?; 
                                }
                                _ => {
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Emissive material has no property '{}'.", key)}));
                                }
                            }
                        }
                        "roughness" => {
                            match &mut mat{
                                Material::MetallicMaterial { albedo: _, roughness} => {
                                    *roughness = value.parse::<f32>()?; 
                                }
                                _ => {
                                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Emissive material has no property '{}'.", key)}));
                                }
                            }
                        }
                        _ => {
                            return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Unimplemented key while parsing material '{}'.", key)}));
                        }
                    }
                }
                else{
                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of line while parsing material.".to_owned()}));
                }
            },
            None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of file while parsing material.".to_string()})); },
        };
    }
    return Ok(mat);
} 

fn parse_mesh_object<'a, I>(lines: &mut Peekable<I>, filename: &str, line_number: &mut usize) -> Result<Mesh, Box<dyn Error>>
where
I: DoubleEndedIterator<Item = &'a str> + Clone{
    let mut object: Option<Mesh> = None;
    match lines.next(){
        Some(line) => {
            if let [key, value] = &line.split("=").map(|x| x.trim()).take(2).collect::<Vec<_>>()[..]{
                match *key{
                    "mesh_file" => {
                        object = Some(Mesh::from_obj(value)?);
                    },
                    _ => {
                        return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Unimplemented key while parsing mesh object '{}'.", key)}));
                    }
                }
            }
            else{
                return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of line while parsing key.".to_owned()}));
            }
        },
        None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of file while parsing mesh object.".to_string()})); },
    };
    if let Some(mut obj) = object{
        obj.material = Material::NormalMaterial();
        return Ok(obj);
    }   
    else{
        return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "No mesh file proveded for mesh file object".to_owned()}));
    }

}

fn parse_sphere_object<'a, I>(lines: &mut Peekable<I>, filename: &str, line_number: &mut usize) -> Result<Sphere, Box<dyn Error>>
where
I: DoubleEndedIterator<Item = &'a str> + Clone{
    let mut pos = Vec3::zero();
    let mut radius = 1.0;
    for _ in 0..2{
        match lines.next(){
            Some(line) => {
                if let [key, value] = &line.split("=").map(|x| x.trim()).take(2).collect::<Vec<_>>()[..]{
                    match *key{
                        "radius" => {
                            radius = value.parse()?;
                        },
                        "pos" => {
                            pos = parse_vec3(value)?;
                        }
                        _ => {
                            return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Unimplemented key while parsing sphere object '{}'.", key)}));
                        }
                    }
                }
                else{
                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of line while parsing key.".to_owned()}));
                }
            },
            None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of file while parsing sphere object.".to_string()})); },
        };
    }

    return Ok(Sphere { center: pos, radius, material: Material::NormalMaterial() });

}

fn parse_camera<'a, I>(lines: &mut Peekable<I>, filename: &str, line_number: &mut usize) -> Result<(Camera, u32, u32), Box<dyn Error>>
where
I: DoubleEndedIterator<Item = &'a str> + Clone{
    let mut cam_params: CameraParameters = CameraParameters::default();
    cam_params.depth_of_field = 1.0;

    let mut width: u32 = 400;
    let mut height: u32 = 225;
    // check if the first char on the next line == '['
    while (*lines.peek().or(Some(&"[]")).unwrap()).chars().nth(0) != Some('['){
        match lines.next(){
            Some(line) => {
                if let [key, value] = &line.split("=").map(|x| x.trim()).take(2).collect::<Vec<_>>()[..]{
                    match *key{
                        "height" => {
                            height = value.trim().parse()?;
                        },
                        "width" => {
                            width = value.trim().parse()?;
                        },
                        "position" => {
                            cam_params.pos = parse_vec3(value)?;
                        },
                        "target" => {
                            cam_params.target = parse_vec3(value)?;
                       },
                        "fov" => {
                            cam_params.fov = value.parse::<f32>()?
                        },
                        _ => {
                            return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: format!("Unimplemented key while parsing camera {}'.", key)}));
                        }
                    }
                }
                else{
                    return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of line while parsing key.".to_owned()}));
                }
            },
            None => { return Err(Box::new(ParsingError{filename: filename.to_owned(), line: *line_number, message: "Hit end of file while parsing mesh object.".to_string()})); },
        };
    }
    let camera = Camera::new(cam_params.pos, cam_params.target, cam_params.fov, width as f32 / height as f32, cam_params.aperture_size, cam_params.depth_of_field);
    return Ok((camera, width, height));

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
