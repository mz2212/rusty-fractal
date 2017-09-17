extern crate sdl2;
extern crate hsv_rgb;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::io;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
	let mut sdl_quit = false;
	let mut calc_brot = true;
	let mut do_render = true;
	let mut max_iter = 256;
	let mut zoom = 1.0;
	let mut move_x = -0.5;
	let mut move_y = 0.0;
	let move_speed = 0.1;
	let mut zoom_speed = 1.25;
	let mut iter_array = [[0; WIDTH as usize +1]; HEIGHT as usize + 1];
	let mut single_hue_mode: bool = false;
	let mut hue: u8 = 220;

	// SDL setup. Go takes care of this bit.
	let sdl_context = sdl2::init().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	// You do the below in Go as well.
	let window = video_subsystem.window("Mandelbrot", WIDTH, HEIGHT)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	
	let mut canvas = window.into_canvas().build().unwrap();

	while !sdl_quit {
		if calc_brot {
			for x in 0..WIDTH {
				for y in 0..HEIGHT {
						iter_array[y as usize][x as usize] = crunch(max_iter, x, y, zoom, move_x, move_y)
				}
			}
			calc_brot = false;
		}

		if do_render {
			canvas.clear();
			for x in 0..WIDTH {
				for y in 0..HEIGHT {
					paint(x, y, iter_array[y as usize][x as usize], max_iter, &mut canvas, single_hue_mode, hue)
				}
			}
			canvas.present();
			do_render = false;
		}

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					sdl_quit = true;
				},
				Event::KeyUp {keycode: key, ..} => {
					match key.unwrap() {
						Keycode::A => {
							move_x -= move_speed / zoom;
							calc_brot = true;
							do_render = true;
						},
						Keycode::D => {
							move_x += move_speed / zoom;
							calc_brot = true;
							do_render = true;
						},
						Keycode::W => {
							move_y -= move_speed / zoom;
							calc_brot = true;
							do_render = true;
						},
						Keycode::S => {
							move_y += move_speed / zoom;
							calc_brot = true;
							do_render = true;
						},
						Keycode::E => {
							zoom *= zoom_speed;
							zoom_speed += 0.05;
							calc_brot = true;
							do_render = true;
						},
						Keycode::Q => {
							zoom /= zoom_speed;
							zoom_speed -= 0.05;
							calc_brot = true;
							do_render = true;
						},
						Keycode::Equals => {
							max_iter *= 2;
							calc_brot = true;
							do_render = true;
						},
						Keycode::Minus => {
							if max_iter > 2 {
								max_iter /= 2;
							}
							calc_brot = true;
							do_render = true;
						},
						Keycode::M => {
							single_hue_mode = !single_hue_mode;
							do_render = true;
						},
						Keycode::H => { // Special, Need to get input from console.
							let mut input_text = String::new();
							println!("Please input a hue (0-255)");
							io::stdin()
								.read_line(&mut input_text)
								.expect("Failed to read from stdin...");
							
							match input_text.trim().parse::<u8>() {
								Ok(i) => {
									println!("New hue: {}", i);
									hue = i;
									do_render = true;
								},
								Err(..) => println!("Please input a number (0-255)")
							};
						},
						_ => {}
					}
				},
				_ => {}
			}
		}
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}

fn paint(x: u32, y: u32, iter: u32, max_iter: u32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, single_hue: bool, hue: u8) {
	// Why doesn't this need to be mutable?
	let color: hsv_rgb::RGBColor;
	if single_hue {
		color = hsv_rgb::HSVColor{
			h: hue,
			s: 255,
			v: (iter % 256) as u8 * (iter < max_iter) as u8,
		}.to_rgb();
	} else {
		color = hsv_rgb::HSVColor{
			h: (iter % 256) as u8,
			s: 255,
			v: 255 * (iter < max_iter) as u8
		}.to_rgb();
	}
	canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
	let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
}

fn crunch(max_iter: u32, x: u32, y: u32, zoom: f64, move_x: f64, move_y: f64) -> u32 {
	let mut old_imag;
	let mut old_real;
	let mut new_imag;
	let mut new_real;
	let mut iter = 0;

	let pix_real = 1.5*(x as f64 - WIDTH as f64/2.0)/(0.5*zoom*WIDTH as f64) + move_x;
	let pix_imag = (y as f64 - HEIGHT as f64/2.0)/(0.5*zoom*HEIGHT as f64) + move_y;
	new_real = 0.0;
	new_imag = 0.0;
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
