ARTIFACTS := clock.png projectile.png projection.png shading.png shading_parallel.png

all: $(ARTIFACTS)

%.png: %.ppm
	convert "$<" "$@"

clean:
	rm $(ARTIFACTS)
