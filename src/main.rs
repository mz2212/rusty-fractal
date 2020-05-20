extern crate sdl2;
extern crate hsv_rgb;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent;
use sdl2::video::FullscreenType;
use std::time::Duration;
use std::io;

mod mandelbrot;
mod julia;

fn main() {
	let mut sdl_quit = false;
	let mut calc = true;
	let mut do_render = true;
	let mut max_iter = 256;
	let mut zoom = 1.0;
	let mut move_x = -0.5;
	let mut move_y = 0.0;
	let move_speed = 0.1;
	let mut zoom_speed = 1.25;
	let mut single_hue_mode: bool = false;
	let mut hue: u8 = 220;
	let mut mode = Mode::Mandelbrot;
	let julia_real = -0.7;
	let julia_imag = 0.27015;
	let max_threads = 32;

	// SDL setup. Go takes care of this bit.
	let sdl_context = sdl2::init().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	// You do the below in Go as well.
	let window = video_subsystem.window("Fractals", 1280, 720)
		.position_centered()
		.resizable()
		.opengl()
		.build()
		.unwrap();
	
	let mut canvas = window.into_canvas().build().unwrap();
	let (mut width, mut height) = canvas.window().size();
	let mut iter_array = vec![0; (width*height) as usize + 1];

	while !sdl_quit {
		if calc {
			match mode {
				Mode::Mandelbrot => {
					//iter_array[y as usize][x as usize] = mandelbrot::crunch(max_iter, x, y, zoom, move_x, move_y, width, height)
					iter_array = mandelbrot::crunch(max_iter, iter_array, zoom, move_x, move_y, width, height, max_threads);
				},
				//Mode::Julia => iter_array[y as usize][x as usize] = julia::crunch(max_iter, x, y, zoom, move_x, move_y, width, height, julia_real, julia_imag)
				_ => {}
			}
			calc = false;
			do_render = true;
		}

		if do_render {
			canvas.clear();
			for q in 0..width*height {
				paint(q%width, q/width, iter_array[q as usize], max_iter, &mut canvas, single_hue_mode, hue)
			}
			canvas.present();
			do_render = false;
		}

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					sdl_quit = true;
				},
				Event::Window {win_event: wevent, ..} => {
					match wevent {
						WindowEvent::Resized {..} | WindowEvent::SizeChanged {..} => {
							width = canvas.window().size().0;
							height = canvas.window().size().1;
							iter_array = vec![0; (width*height) as usize + 1];
							calc = true;
						},
						_ => {}
					}
				},
				Event::KeyUp {keycode: key, ..} => {
					match key.unwrap() {
						Keycode::A | Keycode::Left => {
							move_x -= move_speed / zoom;
							calc = true;
						},
						Keycode::D | Keycode::Right => {
							move_x += move_speed / zoom;
							calc = true;
						},
						Keycode::W | Keycode::Up => {
							move_y -= move_speed / zoom;
							calc = true;
						},
						Keycode::S | Keycode::Down => {
							move_y += move_speed / zoom;
							calc = true;
						},
						Keycode::E | Keycode::PageUp => {
							zoom *= zoom_speed;
							zoom_speed += 0.05;
							calc = true;
						},
						Keycode::Q | Keycode::PageDown => {
							zoom /= zoom_speed;
							zoom_speed -= 0.05;
							calc = true;
						},
						Keycode::Equals => {
							max_iter *= 2;
							calc = true;
						},
						Keycode::Minus => {
							if max_iter > 2 {
								max_iter /= 2;
							}
							calc = true;
						},
						Keycode::M => {
							single_hue_mode = !single_hue_mode;
							do_render = true;
						},
						Keycode::F => {
							if canvas.window().fullscreen_state() == FullscreenType::Off {
								let _ = canvas.window_mut().set_fullscreen(FullscreenType::Desktop);
							} else {
								let _ = canvas.window_mut().set_fullscreen(FullscreenType::Off);
							}
						},
						Keycode::Num1 => {
							mode = Mode::Mandelbrot;
							zoom = 1.0;
							move_x = -0.5;
							move_y = 0.0;
							calc = true;
						},
						Keycode::Num2 => {
							mode = Mode::Julia;
							zoom = 1.0;
							move_x = 0.0;
							move_y = 0.0;
							calc = true;
						},
						Keycode::H => { // Special, Need to get input from console.
							let mut input_text = String::new();
							println!("Please input a hue (0-360)");
							io::stdin()
								.read_line(&mut input_text)
								.expect("Failed to read from stdin...");
							
							match input_text.trim().parse::<u16>() {
								Ok(i) => {
									println!("New hue: {}", i);
									hue = ((i as f32/360 as f32)*255 as f32).floor() as u8;
									do_render = true;
								},
								Err(..) => println!("Please input a number (0-360)")
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

enum Mode {
	Mandelbrot,
	Julia,
}
