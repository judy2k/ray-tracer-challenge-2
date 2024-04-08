use std::{error::Error, fs::OpenOptions, io::BufWriter, time::Instant};

use ray_tracer_challenge_2::{
    canvas::Canvas, color::Color, matrix::Matrix, ray::{Intersections, Ray}, shape::Sphere, space::Tuple
};

const OUTPUT_PATH: &str = "output/projection.ppm";

fn main() -> Result<(), Box<dyn Error>>{
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 180;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();
    *shape.transformation() = Matrix::scaling(0.5, 1.0, 1.0);

    let before = Instant::now();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(origin, (position - origin).normalize());
            // TODO: Optimize this - shouldn't transfer from vector to Intersections.
            // Pass in &mut Intersections?
            let xs = shape.intersect(r);
            let mut is = Intersections::new();
            for x in xs.iter() {
                is.add(x);
            }
            if is.hit().is_some() {
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
