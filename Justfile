test:
    cargo nextest run

run target="projectiles":
    cargo run --example {{target}}

examples:
    just run plot_projectile
    just run clock
