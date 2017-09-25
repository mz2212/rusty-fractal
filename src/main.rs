#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;


fn main() {
	let loc: [f32; 2] = [-0.5, 0.0];
	let mut width: u32;
	let mut height: u32;
	let max_iter = 300;
	let zoom: f32 = 1.0;
	let mut render = true;

	let mut path = PathBuf::new();
	path.push(std::env::current_exe().unwrap());
	path.set_file_name("hsvrgb.frag");
	let mut file = OpenOptions::new()
		.read(true)
		.open(path.as_path()).unwrap();
	let mut brot_src = String::new();
	let _ = file.read_to_string(&mut brot_src);
	drop(file);
	path.set_file_name("mandelbrot.frag");
	let mut file = OpenOptions::new()
		.read(true)
		.open(path.as_path()).unwrap();
	let _ = file.read_to_string(&mut brot_src);
	drop(file);

	// GL setup
	let mut event_loop = glutin::EventsLoop::new();
	let window = glutin::WindowBuilder::new();
	let context = glutin::ContextBuilder::new();
	let display = glium::Display::new(window, context, &event_loop).unwrap();
	
	// Set up square we're drawing to with pixel shader
	// For some reason I can't draw quads with glium
	implement_vertex!(Vertex, pos);
	let vert1 = Vertex { pos: [-1.0, -1.0] };
    let vert2 = Vertex { pos: [1.0, -1.0] };
    let vert3 = Vertex { pos: [1.0, 1.0] };
    let vert4 = Vertex { pos: [-1.0, -1.0] };
    let vert5 = Vertex { pos: [-1.0, 1.0] };
    let vert6 = Vertex { pos: [1.0, 1.0] };
    let shape = vec![vert1, vert2, vert3, vert4, vert5, vert6];

	// Vertex Buffer and indices
	let vert_buf = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	// This is going to be the same for every fractal
	let vertex_shader_src = r#"
		#version 140

		in vec2 pos;

		void main() {
			gl_Position = vec4(pos, 0.0, 1.0);
		}
	"#;

	let gpu_program = glium::Program::from_source(&display, vertex_shader_src, &brot_src, None).unwrap();

	// Event loop
	let mut closed = false;
	while !closed {
		if render {
			let mut target = display.draw();
			width = target.get_dimensions().0;
			height = target.get_dimensions().1;
			target.clear_color(1.0, 0.0, 0.0, 1.0);
			let uni = uniform! {
				loc: loc, 
				max_iter: max_iter, 
				zoom: zoom,
				width: width,
				height: height,
			};
			target.draw(&vert_buf, &indices, &gpu_program, &uni, &Default::default()).unwrap();
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
				_ => {}
			}
		});
	}
}

#[derive(Copy, Clone)]
struct Vertex {
	pos: [f32; 2],
}
