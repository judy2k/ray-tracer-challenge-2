use crate::{lighting::PointLight, shape::Shape};


pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Shape>
}