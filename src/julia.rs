pub fn calc(max_iter: u32, x: u32, y: u32, zoom: f64, move_x: f64, move_y: f64, width: u32, height: u32) -> u32 {
	let mut old_real;
	let mut old_imag;
	let mut new_real = (1.5 * (x as f64) - width as f64 / 2.0) / (0.5 * zoom * width as f64) + move_x;
	let mut new_imag = ((y as f64) - height as f64 / 2.0) / (0.5 * zoom * height as f64) + move_y;
	let mut iter = 0;
	let julia_real = -0.7;
	let julia_imag = 0.27015;

	for i in 0..=max_iter {
		iter = i;
		old_real = new_real;
		old_imag = new_imag;

		new_real = old_real * old_real - old_imag * old_imag + julia_real;
		new_imag = 2.0 * old_real * old_imag + julia_imag;
		if new_real * new_real + new_imag * new_imag > 4.0 {
			break
		}
	}
	iter
}