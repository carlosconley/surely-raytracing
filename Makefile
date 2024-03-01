
.PHONEY: build, run

build:
	cargo build --release

run:
	-cargo run --release > $(file).ppm;
	convert $(file).ppm $(file).png

clean: 
	-rm -rf *.ppm *.png target/