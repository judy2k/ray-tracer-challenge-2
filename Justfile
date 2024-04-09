test:
    cargo nextest run --no-fail-fast

run target="projectiles":
    cargo run --release --example {{target}}

examples:
    just run plot_projectile
    just run clock
    just run projection
    make -f output/Makefile