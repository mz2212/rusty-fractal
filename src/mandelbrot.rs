use std::thread;

pub fn crunch(max_iter: u32, pixels: Vec<u32>, zoom: f64, move_x: f64, move_y: f64, width: u32, height: u32, max_threads: u32) -> Vec<u32>{
	let mut threads: Vec<thread::JoinHandle<Vec<u32>>> = Vec::new();
	let pixels_to_calculate: u32 = pixels.len() as u32/ max_threads;
	let mut out: Vec<u32> = Vec::new();
	for i in 1..=max_threads {
		threads.push(thread::spawn(move || {
			let mut thread_chunk: Vec<u32> = Vec::new();
			for q in pixels_to_calculate*i.saturating_sub(1)..pixels_to_calculate*i {
				thread_chunk.push(calc(max_iter, q % width, q/width, zoom, move_x, move_y, width, height));
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

fn calc(max_iter: u32, x: u32, y: u32, zoom: f64, move_x: f64, move_y: f64, width: u32, height: u32) -> u32 {
	let mut old_imag;
	let mut old_real;
	let mut new_imag = 0.0;
	let mut new_real = 0.0;
	let mut iter = 0;

	let pix_real = 1.5*(x as f64 - width as f64/2.0)/(0.5*zoom*width as f64) + move_x;
	let pix_imag = (y as f64 - height as f64/2.0)/(0.5*zoom*height as f64) + move_y;
	for i in 0..max_iter + 1 {
		iter = i;
		old_real = new_real;
		old_imag = new_imag;

		new_real = old_real*old_real - old_imag*old_imag + pix_real;
		new_imag = 2.0*old_real*old_imag + pix_imag;
		if new_real*new_real+new_imag*new_imag > 4.0 {
			break
		}
	}
	iter
}