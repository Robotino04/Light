use ultraviolet::Vec3;

pub fn gamma_correct(gamma: f32, color: Vec3) -> Vec3{
    Vec3{
        x: color.x.powf(1.0/gamma),
        y: color.y.powf(1.0/gamma),
        z: color.z.powf(1.0/gamma),
    }
}

pub fn average_samples(num_samples: usize, color: Vec3) -> Vec3{
    color / num_samples as f32
}
