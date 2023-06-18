use crate::{camera::Camera, hittable::Hittable};

#[derive(Default)]
pub struct Scene{
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>,
}
