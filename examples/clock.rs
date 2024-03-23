use ray_tracer_challenge_2::canvas::Canvas;
use ray_tracer_challenge_2::color::Color;
use ray_tracer_challenge_2::matrix::Matrix;
use ray_tracer_challenge_2::space::Tuple;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::OpenOptions;
use std::io::BufWriter;

const POINT_COUNT: usize = 32;
const OUTPUT_PATH: &str = "output/clock.ppm";

fn main() -> Result<(), Box<dyn Error>> {
    let point_color = Color::new(1.0, 0., 0.);

    let mut canvas = Canvas::new(550, 550);

    for i in 0..POINT_COUNT {
        let angle = 2.0 * PI / POINT_COUNT as f64 * i as f64;
        let centre_x = canvas.width as f64 / 2.0;
        let centre_y = canvas.height as f64 / 2.0;

        let point = Matrix::translation(centre_x, centre_y, 0.0)
            * Matrix::rotation_z(angle)
            * Matrix::translation(0.0, centre_y * 0.9, 0.0)
            * Tuple::point(0.0, 0.0, 0.0);
        canvas.plot_point(&point, &point_color);
    }

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
