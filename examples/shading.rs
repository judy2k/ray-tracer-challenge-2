use std::{error::Error, fs::OpenOptions, io::BufWriter, time::Instant};

use ray_tracer_challenge_2::{
    canvas::Canvas,
    color::Color,
    lighting::PointLight,
    ray::{Intersections, Ray},
    shape::{Shape, Sphere},
    space::Point,
};

const OUTPUT_PATH: &str = "output/shading.ppm";

fn main() -> Result<(), Box<dyn Error>> {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 512;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let mut shape = Sphere::new();
    shape.material().ambient = 0.5;
    shape.material().shininess = 10.0;
    shape.material().color = Color::new(1.0, 0.1, 0.0);
    let mut shape: Shape = shape.into();

    let light_position = Point::new(-10., 10., -10.);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    let before = Instant::now();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let r = Ray::new(origin, (position - origin).normalize());

            let mut is = Intersections::new();
            shape.intersect(&r, &mut is);

            if let Some(hit) = is.hit() {
                let point = r.position(hit.t);
                let normal = shape.normal_at(&point);
                let eye = r.direction * -1.0;
                let color = shape.material().lighting(&light, &point, &eye, &normal);

                canvas.write_pixel(x, y, color)
            }
        }
    }
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
