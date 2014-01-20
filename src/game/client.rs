
#[feature(globs)];


extern mod native;
extern mod glfw;
extern mod gl;
extern mod nalgebra;

use client::app;

mod shared;

pub mod client {
	pub mod app;
	pub mod engine;
	pub mod shader;
	pub mod tilemap;
	pub mod math;
}


#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread
    native::start(argc, argv, main)
}

fn main() {
	app::App::run();
}
