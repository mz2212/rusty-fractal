#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glium::glutin::VirtualKeyCode;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
	let mut loc: [f64; 2] = [-0.5, 0.0];
	let julia_data: [f64; 2] = [-0.7, 0.27015];
	let mut width: u32;
	let mut height: u32;
	let mut max_iter = 256;
	let mut zoom: f64 = 1.0;
	let mut render = true;
	let move_speed = 0.1;
	let mut zoom_speed = 1.25;
	let mut mode = Mode::Mandelbrot;

	// I wish glutin came with a way to load shaders from files...
	let mut path = PathBuf::new();
	path.push(std::env::current_exe().unwrap());
	path.set_file_name("hsvrgb.frag");
	let mut file = OpenOptions::new()
		.read(true)
		.open(path.as_path()).unwrap();
	let mut brot_src = String::new();
	let mut julia_src = String::new();
	let _ = file.read_to_string(&mut brot_src);
	julia_src = brot_src.clone();
	drop(file);
	path.set_file_name("mandelbrot.frag");
	let mut file = OpenOptions::new()
		.read(true)
		.open(path.as_path()).unwrap();
	let _ = file.read_to_string(&mut brot_src);
	drop(file);
	path.set_file_name("julia.frag");
	let mut file = OpenOptions::new()
		.read(true)
		.open(path.as_path()).unwrap();
	let _ = file.read_to_string(&mut julia_src);
	drop(file);

	// GL setup
	let mut event_loop = glutin::EventsLoop::new();
	let window = glutin::WindowBuilder::new();
	let context = glutin::ContextBuilder::new()
		.with_multisampling(0);
	let display = glium::Display::new(window, context, &event_loop).unwrap();
	
	// Set up square we're drawing to with pixel shader
	// For some reason I can't draw quads with glium
	implement_vertex!(Vertex, pos);
	let vert1 = Vertex { pos: [-1.0, -1.0] };
	let vert2 = Vertex { pos: [-1.0, 3.0] };
	let vert3 = Vertex { pos: [3.0, -1.0] };
	let shape = vec![vert1, vert2, vert3];

	// Vertex Buffer and indices
	let vert_buf = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	// This is going to be the same for every fractal
	let vertex_shader_src = r#"
		#version 450

		in vec2 pos;

		void main() {
			gl_Position = vec4(pos, 0.0, 1.0);
		}
	"#;

	let brot_program = glium::Program::from_source(&display, vertex_shader_src, &brot_src, None).unwrap();
	let julia_program = glium::Program::from_source(&display, vertex_shader_src, &julia_src, None).unwrap();

	// Event loop
	let mut closed = false;
	while !closed {
		if render {
			let mut target = display.draw();
			width = target.get_dimensions().0;
			height = target.get_dimensions().1;
			target.clear_color(1.0, 0.0, 0.0, 1.0);
			let brot_uni = uniform! {
				loc: loc, 
				max_iter: max_iter, 
				zoom: zoom,
				width: width,
				height: height,
			};
			let julia_uni = uniform! {
				loc: loc,
				max_iter: max_iter,
				zoom: zoom,
				width: width,
				height: height,
				julia_data: julia_data,
			};
			match mode {
				Mode::Mandelbrot => target.draw(&vert_buf, &indices, &brot_program, &brot_uni, &Default::default()).unwrap(),
				Mode::Julia => target.draw(&vert_buf, &indices, &julia_program, &julia_uni, &Default::default()).unwrap()
			}
			target.finish().unwrap();
			render = false;
		}
		event_loop.poll_events(|ev| {
			match ev {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::Closed => { closed = true },
					glutin::WindowEvent::Resized {..} => { render = true },
					_ => {}
				},
				glutin::Event::DeviceEvent { event, .. } => match event {
					glutin::DeviceEvent::Key { 0: input } => {
						if input.state == glutin::ElementState::Released {
							match input.virtual_keycode.unwrap_or(VirtualKeyCode::B) {
								VirtualKeyCode::A | VirtualKeyCode::Left => {
									loc[0] -= move_speed / zoom;
									render = true;
								},
								VirtualKeyCode::D | VirtualKeyCode::Right => {
									loc[0] += move_speed / zoom;
									render = true;
								},
								VirtualKeyCode::S | VirtualKeyCode::Down => {
									loc[1] -= move_speed / zoom;
									render = true;
								},
								VirtualKeyCode::W | VirtualKeyCode::Up => {
									loc[1] += move_speed / zoom;
									render = true;
								},
								VirtualKeyCode::Equals => {
									if max_iter < 4096 {
										max_iter *= 2;
										render = true;
									}
								},
								VirtualKeyCode::Underline | VirtualKeyCode::Key0 => {
									if max_iter > 2 {
										max_iter /= 2;
										render = true;
									}
								},
								VirtualKeyCode::E | VirtualKeyCode::PageUp => {
									zoom *= zoom_speed;
									zoom_speed += 0.05;
									render = true;
								},
								VirtualKeyCode::Q | VirtualKeyCode::PageDown => {
									zoom /= zoom_speed;
									zoom_speed -= 0.05;
									render = true;
								},
								VirtualKeyCode::Key1 => {
									mode = Mode::Mandelbrot;
									zoom = 1.0;
									loc = [-0.5, 0.0];
									render = true;
								},
								VirtualKeyCode::Key2 => {
									mode = Mode::Julia;
									zoom = 1.0;
									loc = [0.0, 0.0];
									render = true;
								},
								_ => {}
							}
						}
					},
					_ => {}
				},
				_ => {}
			}
		});
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}

enum Mode {
	Julia,
	Mandelbrot,
}

#[derive(Copy, Clone)]
struct Vertex {
	pos: [f32; 2],
}
