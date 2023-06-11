use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material{
    NormalMaterial(),
    DiffuseMaterial{albedo: Vec3},
    MetallicMaterial{albedo: Vec3, roughness: f32}, 
    DielectricMaterial{albedo: Vec3, ior: f32},
}
