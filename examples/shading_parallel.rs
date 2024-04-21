use std::{error::Error, fs::OpenOptions, io::BufWriter, time::Instant};
use rayon::prelude::*;
use ray_tracer_challenge_2::{
    canvas::Canvas,
    color::Color,
    lighting::PointLight,
    ray::{Intersections, Ray},
    shape::{Shape, Sphere},
    space::Point,
};

const OUTPUT_PATH: &str = "output/shading_parallel.ppm";

fn generate_pixel(ray: &Ray, shape: &Shape, light: &PointLight) -> Option<Color> {
    let mut is = Intersections::new();
    shape.intersect(&ray, &mut is);

    if let Some(hit) = is.hit() {
        let point = ray.position(hit.t);
        let normal = shape.normal_at(&point);
        let eye = ray.direction * -1.0;
        let color = shape.material().lighting(&light, &point, &eye, &normal);

        return Some(color);
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 512;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let mut shape = Sphere::new();
    let material = shape.material_mut();
    material.ambient = 0.5;
    material.shininess = 10.0;
    material.color = Color::new(1.0, 0.1, 0.0);
    let shape: Shape = shape.into();

    let light_position = Point::new(-10., 10., -10.);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    let before = Instant::now();

    (0..canvas_pixels).flat_map(|y| -> Vec<_> {
        let world_y = half - pixel_size * y as f64;
        let sh = shape.clone();
        let l = light.clone();
        (0..canvas_pixels as usize).into_par_iter().filter_map(move |x| {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            if let Some(color) = generate_pixel(&ray, &sh, &l) {
                Some((x, y, color))
            } else {
                None
            }
        }).collect()
    }).for_each(|(x, y, color)| canvas.write_pixel(x, y, color));

    println!("Generated pixels in {:.2?}", before.elapsed());
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(OUTPUT_PATH)?,
    );
    println!("Writing file... {}", OUTPUT_PATH);
    canvas.write_ppm(&mut file)?;
    println!("Done.");

    Ok(())
}
