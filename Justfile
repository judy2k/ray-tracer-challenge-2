test:
    cargo nextest run

run target="projectiles":
    cargo run --example {{target}}
