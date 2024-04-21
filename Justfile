test:
    cargo nextest run --no-fail-fast

run target="shading":
    cargo run --release --example {{target}}
    make -C output

examples:
    just run plot_projectile
    just run clock
    just run projection
    just run shading
    just run shading_parallel

clean:
    make -C output clean