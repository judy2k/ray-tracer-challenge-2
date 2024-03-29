use ray_tracer_challenge_2::canvas::Canvas;
use ray_tracer_challenge_2::color::Color;
use ray_tracer_challenge_2::space::{Point, Tuple, Vector};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufWriter;

const OUTPUT_PATH: &str = "output/projectile.ppm";

#[derive(Debug)]
struct Env {
    gravity: Vector,
    wind: Vector,
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

fn tick(env: &Env, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + env.gravity + env.wind,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let projectile_color = Color::new(1.0, 0., 0.);

    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let environment = Env {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while projectile.position.y() >= 1.0 {
        let canvas_x = projectile.position.x().round() as usize;
        let canvas_y = canvas.height - projectile.position.y().round() as usize;

        canvas.plot_point(
            &Tuple::point(canvas_x as f64, canvas_y as f64, 0.0),
            &projectile_color,
        );

        projectile = tick(&environment, projectile);
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
