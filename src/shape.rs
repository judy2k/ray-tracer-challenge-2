use crate::matrix::{identity_matrix, Matrix};
use crate::ray::{Intersection, Intersections};
use crate::space::{Point, Vector};
use crate::{ray::Ray};

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

    pub fn with_transform(transformation: Matrix) -> Self {
        Self { transformation }
    }

    pub fn transformation(&mut self) -> &mut Matrix {
        &mut self.transformation
    }

    pub fn intersect<'a>(&'a self, ray: Ray, intersections: &mut Intersections<'a>) {
        let ray2 = ray.transform(&self.transformation.inverse().unwrap());

        let sphere_to_ray = ray2.origin - Point::new(0., 0., 0.);
        let a = ray2.direction.dot(ray2.direction);
        let b = 2. * ray2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;

        if discriminant >= 0.0 {
            intersections.add(Intersection::new(
                (-b - discriminant.sqrt()) / (2. * a),
                self.into(),
            ));
            intersections.add(Intersection::new(
                (-b + discriminant.sqrt()) / (2. * a),
                self.into(),
            ));
        }
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let it = self.transformation.inverse().unwrap();
        let op = &it * p;
        let on = op.subtract_origin();
        let wn = it.transpose() * on;
        wn.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new()
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{ray::Ray, space::Vector};

    use super::*;

    #[test]
    fn test_intersect_sphere() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();

        let mut is = Intersections::new();
        s.intersect(r, &mut is);
        assert_eq!(is.len(), 2);
        let mut is_iter = is.into_iter();
        let i = is_iter.next().expect("First intersection");
        assert_eq!(i.t, 4.0);
        assert_eq!(i.shape, Shape::Sphere(&s));
        let i = is_iter.next().expect("Second intersection");
        assert_eq!(i.t, 6.0);
        assert_eq!(i.shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersect_sphere_tangent() {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();

        let mut is = Intersections::new();
        s.intersect(r, &mut is);
        assert_eq!(is.len(), 2);

        let mut is_iter = is.into_iter();
        let i = is_iter.next().expect("First intersection");
        assert_eq!(i.t, 5.0);
        assert_eq!(i.shape, Shape::Sphere(&s));
        let i = is_iter.next().expect("Second intersection");
        assert_eq!(i.t, 5.0);
        assert_eq!(i.shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersect_sphere_miss() {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();

        let mut is = Intersections::new();
        s.intersect(r, &mut is);
        assert_eq!(is.len(), 0);
    }

    #[test]
    fn test_intersect_sphere_from_inside() {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let mut is = Intersections::new();
        s.intersect(r, &mut is);
        assert_eq!(is.len(), 2);

        let mut is_iter = is.into_iter();
        let i = is_iter.next().expect("First intersection");
        assert_eq!(i.t, -1.0);
        assert_eq!(i.shape, Shape::Sphere(&s));

        let i = is_iter.next().expect("Second intersection");
        assert_eq!(i.t, 1.0);
        assert_eq!(i.shape, Shape::Sphere(&s));
    }

    #[test]
    fn test_intersect_sphere_from_behind() {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();

        let mut is = Intersections::new();
        s.intersect(r, &mut is);
        assert_eq!(is.len(), 2);

        let mut is_iter = is.into_iter();
        let i = is_iter.next().expect("First intersection");
        assert_eq!(i.t, -6.0);
        assert_eq!(i.shape, Shape::Sphere(&s));

        let i = is_iter.next().expect("Second intersection");
        assert_eq!(i.t, -4.0);
        assert_eq!(i.shape, Shape::Sphere(&s));
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
    fn test_with_transformation() {
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let s = Sphere::with_transform(t.clone());
        assert_eq!(s.transformation, t);
    }

    #[test]
    fn test_intersect_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transformation = Matrix::scaling(2.0, 2.0, 2.0);

        let mut is = Intersections::new();
        s.intersect(r, &mut is);
        assert_eq!(is.len(), 2);

        let mut is_iter = is.into_iter();
        let i = is_iter.next().expect("First intersection");
        assert_eq!(i.t, 3.0);
        let i = is_iter.next().expect("Second intersection");
        assert_eq!(i.t, 7.0);
    }

    #[test]
    fn test_intersect_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transformation = Matrix::translation(5.0, 0.0, 0.0);

        let mut is = Intersections::new();
        s.intersect(r, &mut is);

        assert_eq!(is.len(), 0);
    }

    #[test]
    fn test_sphere_normal_x() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_normal_y() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_sphere_normal_z() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_normal_nonaxial() {
        // Third root three
        let trt = (3.0_f64).sqrt() / 3.0;
        let s = Sphere::new();
        let n = s.normal_at(Point::new(trt, trt, trt));
        assert_eq!(n, Vector::new(trt, trt, trt));
    }

    #[test]
    fn test_sphere_normal_is_normalized() {
        let trt = (3.0_f64).sqrt() / 3.0;
        let s = Sphere::new();
        let n = s.normal_at(Point::new(trt, trt, trt));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_sphere_translated_normal() {
        let s = Sphere::with_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_sphere_transformed() {
        let s =
            Sphere::with_transform(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0));
        let n = s.normal_at(Point::new(
            0.0,
            (2.0_f64).sqrt() / 2.0,
            -(2.0_f64).sqrt() / 2.0,
        ));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
