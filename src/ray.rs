use crate::shape::Shape;
use crate::space::{Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, d: f64) -> Point {
        self.origin + self.direction * d
    }
}

pub struct Intersection<'a> {
    pub t: f64,
    pub shape: Shape<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: Shape<'a>) -> Self {
        Self { t, shape }
    }
}

#[cfg(test)]
mod test {
    use crate::shape::{Shape, Sphere};
    use crate::space::Tuple;

    use super::{Intersection, Ray};

    #[test]
    fn test_ray_construction() {
        let p = Tuple::point(1., 2., 3.);
        let v = Tuple::vector(4., 5., 6.);

        let r = Ray::new(p, v);
        assert_eq!(r.origin, Tuple::point(1., 2., 3.));
        assert_eq!(r.direction, Tuple::vector(4., 5., 6.));
    }

    #[test]
    fn test_point_computation() {
        let p = Tuple::point(2., 3., 4.);
        let v = Tuple::vector(1., 0., 0.);
        let r = Ray::new(p, v);

        assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));
        assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));
        assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn test_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, (&s).into());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.shape, Shape::Sphere(&s));
    }
}
