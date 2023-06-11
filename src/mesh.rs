use std::{fs::read_to_string, error::Error, usize};

use ultraviolet::{Vec3, Vec2, Mat4};

use crate::{triangle::Triangle, material::Material, hittable::Hittable, ray::Ray, hit_result::HitResult};

pub struct Mesh{
    pub triangles: Vec<Triangle>,
    pub material: Material,
}

impl Mesh {
    pub fn apply_matrix(&mut self, matrix: Mat4){
        self.triangles.iter_mut().for_each(|triangle| {
            triangle.vertices.iter_mut().for_each(|vertex|{
                *vertex = matrix.transform_point3(*vertex);
            });
        });
    }
    pub fn from_obj(filename: &str) -> Result<Mesh, Box<dyn Error>>{
        let mut mesh = Mesh{
            triangles: Vec::new(),
            material: Material::NormalMaterial(),
        };

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut uvs: Vec<Vec2> = Vec::new();
        

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
                    });
                }
                Some("f") => {
                    // vertices
                    let mut triangle = Triangle::default();
                    for i in 0..3{
                        let mut indices = line_parts.next().unwrap().split("/");
                        triangle.vertices[i] = vertices[indices.next().unwrap().parse::<usize>()? - 1];
                        triangle.uv_coordinates[i] = uvs[indices.next().unwrap().parse::<usize>()? - 1];
                        triangle.normals[i] = normals[indices.next().unwrap().parse::<usize>()? - 1];
                    }


                    mesh.triangles.push(triangle);
                }
                Some("#") => {},
                Some(rest) => {
                    println!("Unhandled line {}", rest);
                }
                None => {},
            }
        }

        return Ok(mesh);
    }
}

impl Hittable for Mesh{
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool{
        let mut did_hit = false;
        for hittable in self.triangles.iter() {
            did_hit |= hittable.hit(ray, hit, min_distance);
        }
        if did_hit{
            hit.material = Some(self.material);
        }
        return did_hit;
    }

}
