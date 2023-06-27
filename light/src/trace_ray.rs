use ultraviolet::Vec3;

use crate::{ray::Ray, hittable::Hittable, hit_result::HitResult, material::Material, random::{random_on_unit_sphere, random_in_unit_sphere}, math_utils::lerp};

fn sample_background_gradient(ray: Ray) -> Vec3{
    let t: f32 = 0.5*(ray.direction.y + 1.0);
    return lerp::<Vec3>(t, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0));
}

fn refract(entering_vector: Vec3, normal: Vec3, ior_quotient: f32) -> Vec3{
    let cos_theta = normal.dot(-entering_vector).min(1.0);
    let r_out_perp = ior_quotient * (entering_vector + cos_theta*normal);
    let r_out_parallel = -(1.0 - r_out_perp.mag_sq()).abs().sqrt() * normal;
    return r_out_perp + r_out_parallel;
}

fn schlick_reflectance(cos_theta: f32, ior_current: f32, ior_new: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (ior_current - ior_new) / (ior_current + ior_new);
    r0 = r0*r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
}

pub fn trace_ray(ray: Ray, scene: &Box<dyn Hittable>, depth: i32) -> Vec3{
    if depth == 0{
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let mut hit: HitResult = HitResult::default();
    
    scene.hit(ray, &mut hit, 1e-4);

    match hit.material {
        Some(mat) => {
            match mat {
                Material::NormalMaterial() => hit.normal*0.5 + Vec3::new(0.5, 0.5, 0.5),
                Material::DiffuseMaterial { albedo } => {
                    let target = hit.normal + random_on_unit_sphere();
                    let new_ray: Ray = Ray{
                        origin: ray.at(hit.t),
                        direction: target.normalized(),
                    };
                    return albedo * trace_ray(new_ray, scene, depth-1)
                }
                Material::MetallicMaterial { albedo, roughness } => {
                    let mut direction = ray.direction.reflected(hit.normal) + roughness * random_in_unit_sphere(); 
                    if direction.dot(hit.normal) <= 0.0{
                        // the ray got reflected back into the object
                        return Vec3::new(0.0, 0.0, 0.0);
                    }
                    direction.normalize();
                    let new_ray: Ray = Ray{
                        origin: ray.at(hit.t),
                        direction,
                    };
                    return albedo * trace_ray(new_ray, scene, depth-1)
                }
                Material::DielectricMaterial { albedo, ior } => {
                    // assume the other material is always air
                    let ior_air = 1.0;

                    let ior_current = if hit.is_front_face {ior_air} else {ior};
                    let ior_new = if hit.is_front_face {ior} else {ior_air};
                    let ior_quotient = ior_current/ior_new;
                    
                    let cos_theta = hit.normal.dot(-ray.direction).min(1.0);
                    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

                    let cannot_refract = ior_quotient * sin_theta > 1.0;
                    let direction = if cannot_refract || schlick_reflectance(cos_theta, ior_current, ior_new) > rand::random::<f32>(){
                        ray.direction.reflected(hit.normal) 
                    }
                    else{
                        refract(ray.direction, hit.normal, ior_quotient)
                    };

                    let new_ray: Ray = Ray{
                        origin: ray.at(hit.t),
                        direction,
                    };
                    return albedo * trace_ray(new_ray, scene, depth-1);
                }
                Material::EmissiveMaterial { emission_color, strength } => {
                    return emission_color * strength;
                }
            }

        },
        None => return 0.0*sample_background_gradient(ray),
    }
}
