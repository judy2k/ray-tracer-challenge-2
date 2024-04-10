ARTIFACTS := clock.png projectile.png projection.png shading.png

all: $(ARTIFACTS)

%.png: %.ppm
	convert "$<" "$@"

clean:
	rm $(ARTIFACTS)
