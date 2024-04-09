use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::matrix::Matrix;
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

    pub fn transform(&self, matrix: &Matrix) -> Ray {
        Ray::new((matrix * (*self.origin)).into(), matrix * self.direction)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub shape: Shape<'a>,
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.t.partial_cmp(&other.t) {
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Less) => Some(Ordering::Greater),
            default => default,
        }
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: Shape<'a>) -> Self {
        Self { t, shape }
    }
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Intersections<'a> {
    items: BinaryHeap<Intersection<'a>>,
}

//impl<'a> IntoIterator for Intersections<'a> {
impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = std::collections::binary_heap::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            items: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, i: Intersection<'a>) {
        self.items.push(i);
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        for i in self.items.iter() {
            if i.t.is_sign_positive() {
                return Some(i);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod test {
    use crate::shape::{Shape, Sphere};
    use crate::space::Tuple;

    use super::*;

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

    #[test]
    fn test_intersections_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, (&s).into());
        let i2 = Intersection::new(2.0, (&s).into());

        let mut xs = Intersections::new();
        xs.add(i2);
        xs.add(i1.clone());
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn test_intersections_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, (&s).into());
        let i2 = Intersection::new(1.0, (&s).into());

        let mut xs = Intersections::new();
        xs.add(i2.clone());
        xs.add(i1);
        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn test_intersections_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, (&s).into());
        let i2 = Intersection::new(-1.0, (&s).into());

        let mut xs = Intersections::new();
        xs.add(i2);
        xs.add(i1);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn test_intersections_more_values() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, (&s).into());
        let i2 = Intersection::new(7.0, (&s).into());
        let i3 = Intersection::new(-3.0, (&s).into());
        let i4 = Intersection::new(2.0, (&s).into());

        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);
        xs.add(i3);
        xs.add(i4.clone());
        assert_eq!(xs.hit(), Some(&i4));
    }

    #[test]
    fn test_ray_translation() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_ray_scaling() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }
}
