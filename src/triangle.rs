use ultraviolet::{Vec3, Vec2};

use crate::{hittable::Hittable, ray::Ray, hit_result::HitResult};

#[derive(Default)]
pub struct Triangle{
    pub vertices: [Vec3; 3], 
    pub normals: [Vec3; 3],
    pub uv_coordinates: [Vec2; 3],
}

impl Hittable for Triangle{
    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hit(&self, ray: Ray, hit: &mut HitResult, min_distance: f32) -> bool{
        const EPSILON: f32 = 1e-5;
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON{
            return false;    // This ray is parallel to this triangle.
        }

        let f = 1.0 / a;
        let s = ray.origin - self.vertices[0];
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0{
            return false;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0{
            return false;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(q);

        if t > min_distance && t < hit.t { // ray intersection
            hit.t = t;
            hit.normal = self.normals[0];
            return true;
        }
        else{ // This means that there is a line intersection but not a ray intersection.
            return false;
        }
    }
}
