use std::{fs::read_to_string, error::Error, usize};

use ultraviolet::{Vec3, Vec2};

use crate::{triangle::Triangle, material::Material, hittable::Hittable, ray::Ray, hit_result::HitResult, bounding_box::BVH};

pub struct Mesh{
    pub triangles: Option<Box<dyn Hittable>>,
    pub material: Material,
}

impl Mesh {
    pub fn from_obj(filename: &str) -> Result<Mesh, Box<dyn Error>>{
        println!("Loading \"{}\"...", filename);
        let mut mesh = Mesh{
            triangles: None,
            material: Material::NormalMaterial(),
        };

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut uvs: Vec<Vec2> = Vec::new();
        
        let mut triangles: Vec<Box<dyn Hittable>> = Vec::new();

        let mut do_normal_smoothing = false;

        for line in read_to_string(filename)?.lines() {
            let mut line_parts = line.split(" ");
            match line_parts.nth(0){
                Some("v") => {
                    vertices.push(Vec3{
                        x: line_parts.next().unwrap().parse::<f32>()?,
                        y: line_parts.next().unwrap().parse::<f32>()?,
                        z: line_parts.next().unwrap().parse::<f32>()?,
                    });
                }
                Some("vt") => {
                    uvs.push(Vec2{
                        x: line_parts.next().unwrap().parse::<f32>()?,
                        y: line_parts.next().unwrap().parse::<f32>()?,
                    });
                }
                Some("vn") => {
                    normals.push(Vec3{
                        x: line_parts.next().unwrap().parse::<f32>()?,
                        y: line_parts.next().unwrap().parse::<f32>()?,
                        z: line_parts.next().unwrap().parse::<f32>()?,
                    }.normalized());
                }
                Some("f") => {
                    // vertices
                    let mut triangle = Triangle::default();
                    for i in 0..3{
                        let part = line_parts.next().unwrap();
                        let mut indices = part.split("/").filter(|x| x.chars().count() != 0);
                        triangle.vertices[i] = vertices[indices.next().unwrap().parse::<usize>()? - 1];
                        if !part.contains("//"){
                            triangle.uv_coordinates[i] = uvs[indices.next().unwrap().parse::<usize>()? - 1];
                        }
                        
                        if do_normal_smoothing{
                            triangle.normals[i] = normals[indices.next().unwrap().parse::<usize>()? - 1];
                        }
                    }
                    if !do_normal_smoothing{
                        let face_normal = (triangle.vertices[1] - triangle.vertices[0]).cross(triangle.vertices[2] - triangle.vertices[0]).normalized();
                        triangle.normals[0] = face_normal;
                        triangle.normals[1] = face_normal;
                        triangle.normals[2] = face_normal;
                    }


                    triangles.push(Box::new(triangle));
                }
                Some("#") => {},
                Some("s") =>{
                    do_normal_smoothing = match line_parts.next().unwrap(){
                        "off" => false,
                        _ => true,
                    }
                }
                Some(rest) => {
                    println!("Unhandled line {}", rest);
                }
                None => {},
            }
        }

        mesh.triangles = Some(BVH::<dyn Hittable>::build_recursive(triangles));

        return Ok(mesh);
    }
}

impl Hittable for Mesh{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool{
        match &self.triangles{
            Some(bvh) => {
                if bvh.hit(ray, hit, min_distance){
                    hit.material = Some(self.material);
                    return true;
                }
                return false;
            },
            None => {return false; },
        }
    }

    fn get_min_bounds(&self) -> Vec3 {
        self.triangles.as_ref().expect("A mesh should always contain triangles").get_min_bounds()
    }
    fn get_max_bounds(&self) -> Vec3 {
        self.triangles.as_ref().expect("A mesh should always contain triangles").get_max_bounds()
    }
}
