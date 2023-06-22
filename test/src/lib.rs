use std::mem::swap;

pub trait Object{
    fn something(&self) -> bool;
}

struct EmptyObject{}

impl Object for EmptyObject{
    fn something(&self) -> bool{
        unreachable!();
    }
}

pub struct BVH<T: ?Sized>{
    left: Box<T>,
    right: Box<T>,
}

impl<T: ?Sized> Object for BVH<T>{
    fn something(&self) -> bool{
        return true;
    }
}

pub fn build_bvh(initial_objects: Vec<Box<dyn Object>>) -> Box<dyn Object>{
    let mut working_objects: Vec<Box<dyn Object>> = initial_objects;
    while working_objects.len() != 1{
        // sort here
        working_objects = working_objects.chunks_mut(2).map(|chunk| -> Box<dyn Object>{
            if chunk.len() == 2{
                let mut x = Box::new(BVH{
                    left: Box::new(EmptyObject{}) as Box<dyn Object>,
                    right: Box::new(EmptyObject{}) as Box<dyn Object>,
                });
                swap(&mut x.left, &mut chunk[0]);
                swap(&mut x.right, &mut chunk[1]);
                return x;
            }
            else{
                let mut x = Box::new(EmptyObject{}) as Box<dyn Object>;
                swap(&mut x, &mut chunk[0]);
                return x;
            }
        }).collect();
    }

    let mut x = Box::new(EmptyObject{}) as Box<dyn Object>;
    swap(&mut x, &mut working_objects[0]);

    return x;
}