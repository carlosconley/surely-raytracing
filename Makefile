
.PHONEY: build, run

build:
	cargo build --release --bin raytracer

run:
	-cargo run --release --bin raytracer > $(file).ppm;
	convert $(file).ppm $(file).png

pi:
	cargo run --bin pi

clean: 
	-rm -rf *.ppm *.png target/
