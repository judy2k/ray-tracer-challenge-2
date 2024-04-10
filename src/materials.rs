use crate::{
    color::Color,
    lighting::PointLight,
    space::{Point, Vector},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Point,
        eyev: &Vector,
        normalv: &Vector,
    ) -> Color {
        let black = Color::new(0.0, 0.0, 0.0);

        let effective_color = self.color * light.intensity();
        let lightv = (&light.position() - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);

        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = black;
            specular = black;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (lightv * -1.0).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = black;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lighting::PointLight,
        space::{Point, Vector},
    };

    use super::*;

    #[test]
    fn test_material() {
        let m: Material = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn test_lighting_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::origin();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_eye_between_light_and_surface_at_45() {
        let m = Material::new();
        let position = Point::origin();

        let sqt = 2.0f64.sqrt() / 2.0;
        let eyev = Vector::new(0.0, sqt, -sqt);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_eye_opp_surface_light_45() {
        let m = Material::new();
        let position = Point::origin();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_eye_in_reflection_path() {
        let m = Material::new();
        let position = Point::origin();

        let sqt = 2.0f64.sqrt() / 2.0;
        let eyev = Vector::new(0.0, -sqt, -sqt);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_light_behind_surface() {
        let m = Material::new();
        let position = Point::origin();

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    // #[test]
    // fn test_light_surface_in_shadow() {
    //     let m = Material::new();
    //     let position = Point::origin();

    //     let eyev = Vector::new(0.0, 0.0, -1.0);
    //     let normalv = Vector::new(0.0, 0.0, -1.0);
    //     let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    //     let result = m.lighting(light, position, eyev, normalv, true);
    //     assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    // }
}
