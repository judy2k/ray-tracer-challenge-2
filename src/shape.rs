use crate::{ray::Ray, space::Tuple};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape<'a> {
    Sphere(&'a Sphere),
}

impl<'a> From<&'a Sphere> for Shape<'a> {
    fn from(value: &'a Sphere) -> Self {
        Self::Sphere(value)
    }
}

pub struct Intersection<'a> {
    t: f64,
    shape: Shape<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: Shape<'a>) -> Self {
        Self { t, shape }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Self
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0., 0., 0.);
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                Intersection {
                    t: (-b - discriminant.sqrt()) / (2. * a),
                    shape: self.into(),
                },
                Intersection {
                    t: (-b + discriminant.sqrt()) / (2. * a),
                    shape: self.into(),
                },
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ray::Ray, space::Tuple};

    use super::*;

    #[test]
    fn test_intersect_sphere() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0].t, 4.0);
        assert_eq!(is[0].shape, Shape::Sphere(&s));
        assert_eq!(is[1].t, 6.0);
        assert_eq!(is[1].shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersect_sphere_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0].t, 5.0);
        assert_eq!(is[0].shape, Shape::Sphere(&s));
        assert_eq!(is[1].t, 5.0);
        assert_eq!(is[1].shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersect_sphere_miss() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let is = s.intersect(r);
        assert_eq!(is.len(), 0);
    }

    #[test]
    fn test_intersect_sphere_from_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0].t, -1.0);
        assert_eq!(is[0].shape, Shape::Sphere(&s));
        assert_eq!(is[1].t, 1.0);
        assert_eq!(is[1].shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersect_sphere_from_behind() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0].t, -6.0);
        assert_eq!(is[0].shape, Shape::Sphere(&s));
        assert_eq!(is[1].t, -4.0);
        assert_eq!(is[1].shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, (&s).into());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.shape, Shape::Sphere(&s));
    }
}
