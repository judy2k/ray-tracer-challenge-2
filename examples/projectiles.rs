use ray_tracer_challenge_2::space::{Point, Vector};
use std::error::Error;

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
    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };
    let environment = Env {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    println!("{:?}", projectile.position);
    while projectile.position.y() >= 0.0 {
        projectile = tick(&environment, projectile);
        println!("{:?}", projectile.position);
    }

    Ok(())
}
