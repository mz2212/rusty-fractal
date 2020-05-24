use std::thread;

use super::mandelbrot;
use super::julia;
use super::Mode;

pub fn crunch(max_iter: u32, pixels: Vec<u32>, zoom: f64, move_x: f64, move_y: f64, width: u32, height: u32, max_threads: u32, mode: Mode) -> Vec<u32>{
	let mut threads: Vec<thread::JoinHandle<Vec<u32>>> = Vec::new();
	let pixels_to_calculate: u32 = pixels.len() as u32/ max_threads;
	let mut out: Vec<u32> = Vec::new();
	for i in 1..=max_threads {
		threads.push(thread::spawn(move || {
			let mut thread_chunk: Vec<u32> = Vec::new();
			match mode {
				Mode::Mandelbrot => { // I'm duplicating this bit of code to hopefully see a touch of optimization.
					for q in pixels_to_calculate*i.saturating_sub(1)..pixels_to_calculate*i {
						thread_chunk.push(mandelbrot::calc(max_iter, q % width, q/width, zoom, move_x, move_y, width, height));
					}
				},
				Mode::Julia => {
					for q in pixels_to_calculate*i.saturating_sub(1)..pixels_to_calculate*i {
						thread_chunk.push(julia::calc(max_iter, q % width, q/width, zoom, move_x, move_y, width, height));
					}
				}
			}
			thread_chunk
		})
		);
	}

	for thread in threads {
		out.extend(thread.join().unwrap());
	}

	out
}
