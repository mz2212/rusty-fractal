debug:
	cargo build -j4
	cp -f src/mandelbrot.frag target/debug/mandelbrot.frag
	cp -f src/hsvrgb.frag target/debug/hsvrgb.frag
	cp -f src/julia.frag target/debug/julia.frag

release:
	cargo build --release -j4
	cp -f src/mandelbrot.frag target/release/mandelbrot.frag
	cp -f src/hsvrgb.frag target/release/hsvrgb.frag
	cp -f src/julia.frag target/release/julia.frag

check:
	cargo check -j4

all: release

clean:
	rm -r target
