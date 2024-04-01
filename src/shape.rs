use crate::matrix::{identity_matrix, Matrix};
use crate::{ray::Ray, space::Tuple};
use crate::ray::Intersection;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape<'a> {
    Sphere(&'a Sphere),
}

impl<'a> From<&'a Sphere> for Shape<'a> {
    fn from(value: &'a Sphere) -> Self {
        Self::Sphere(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    transformation: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transformation: identity_matrix().to_owned(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray2 = ray.transform(&self.transformation.inverse().unwrap());

        let sphere_to_ray = ray2.origin - Tuple::point(0., 0., 0.);
        let a = ray2.direction.dot(ray2.direction);
        let b = 2. * ray2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                Intersection::new(
                    (-b - discriminant.sqrt()) / (2. * a),
                    self.into(),
                ),
                Intersection::new(
                    (-b + discriminant.sqrt()) / (2. * a),
                    self.into(),
                ),
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
    fn test_sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(&s.transformation, identity_matrix());
    }

    #[test]
    fn test_new_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.transformation = t.clone();
        assert_eq!(s.transformation, t);
    }

    #[test]
    fn test_intersect_scaled_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transformation = Matrix::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_intersect_translated_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transformation = Matrix::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }
}
