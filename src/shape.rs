use crate::{ray::Ray, space::Tuple};

pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Self
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        /*
        “​ sphere_to_ray ← ray.origin - point(0, 0, 0)​ 
        ​ a ← dot(ray.direction, ray.direction)​ 
        b ← 2 * dot(ray.direction, sphere_to_ray)​ 
        c ← dot(sphere_to_ray, sphere_to_ray) - 1​ 
        ​ discriminant ← b² - 4 * a * c”
 */
        let sphere_to_ray = ray.origin - Tuple::point(0.,0.,0.);
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2. * a),
                (-b + discriminant.sqrt()) / (2. * a),

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
        let r = Ray::new(Tuple::point(0.,0.,-5.), Tuple::vector(0.,0.,1.));
        let s = Sphere::new();

        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], 4.0);
        assert_eq!(is[1], 6.0);
    }

    #[test]
    fn test_intersect_sphere_tangent() {
        let r = Ray::new(Tuple::point(0.,1.,-5.), Tuple::vector(0.,0.,1.));
        let s = Sphere::new();

        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], 5.0);
        assert_eq!(is[1], 5.0);
    }

    #[test]
    fn test_intersect_sphere_miss() {
        let r = Ray::new(Tuple::point(0.,2.,-5.), Tuple::vector(0.,0.,1.));
        let s = Sphere::new();
        let is = s.intersect(r);
        assert_eq!(is.len(), 0);
    }

    #[test]
    fn test_intersect_sphere_from_inside() {
        let r = Ray::new(Tuple::point(0.,0.,0.), Tuple::vector(0.,0.,1.));
        let s = Sphere::new();
        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], -1.0);
        assert_eq!(is[1], 1.0);
    }

        #[test]
    fn test_intersect_sphere_from_behind() {
        let r = Ray::new(Tuple::point(0.,0.,5.), Tuple::vector(0.,0.,1.));
        let s = Sphere::new();
        let is = s.intersect(r);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], -6.0);
        assert_eq!(is[1], -4.0);
    }
}