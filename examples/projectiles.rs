use ray_tracer_challenge_2::space::Tuple;
use std::error::Error;

#[derive(Debug)]
struct Env {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Env, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + env.gravity + env.wind,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };
    let environment = Env {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    println!("{:?}", projectile.position);
    while projectile.position.y() >= 0.0 {
        projectile = tick(&environment, projectile);
        println!("{:?}", projectile.position);
    }

    Ok(())
}
