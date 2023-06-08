use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material{
    NormalMaterial(),
    DiffuseMaterial{albedo: Vec3},
    MetallicMaterial{albedo: Vec3, roughness: f32, metalness: f32}, 
}
