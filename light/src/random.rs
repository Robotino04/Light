use ultraviolet::Vec3;

pub fn random_in_unit_sphere() -> Vec3{
    loop{  
        let v = Vec3{x: rand::random(), y: rand::random(), z: rand::random()} * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if v.mag_sq() < 1.0{
            return v;
        }
    }
}

pub fn random_on_unit_sphere() -> Vec3{
    random_in_unit_sphere().normalized()
}

