use crate::{lighting::PointLight, shape::Shape};


#[derive(Debug, PartialEq, Clone)]
pub struct World {
    light: Option<PointLight>,
    objects: Vec<Shape>
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{color::Color, matrix::Matrix, shape::Sphere, space::Point};

    use super::*;

    #[test]
    fn test_world_init() {
        let w = World::new();

        assert_eq!(w.light, None);
        assert_eq!(w.objects.len(), 0);
    }

    fn default_world() -> World {
        let mut world = World::new();
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        let material = s1.material_mut();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        let mut s2 = Sphere::new();
        *s2.transformation() = Matrix::scaling(0.5, 0.5, 0.5);
        world.light = Some(light);
        world.objects.push(s1.into());
        world.objects.push(s2.into());

        world
    }

    #[test]
    fn test_default_world() {
        let w = default_world();

        assert!(w.light.is_some());
        assert_eq!(w.objects.len(), 2);

        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(Some(light), w.light);

        let mut s1 = Sphere::new();
        let material = s1.material_mut();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        
        let mut s2 = Sphere::new();
        *s2.transformation() = Matrix::scaling(0.5, 0.5, 0.5);

        assert_eq!(<Sphere as Into<Shape>>::into(s1), w.objects[0]);
        assert_eq!(<Sphere as Into<Shape>>::into(s2), w.objects[1]);
    }
}