use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material{
    NormalMaterial(),
    DiffuseMaterial{albedo: Vec3, reflectivity: f32},
}
