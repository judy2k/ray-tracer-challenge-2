use crate::{color::Color, space::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_light() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);

        let light = PointLight::new(position.clone(), intensity.clone());
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
