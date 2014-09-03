
#![feature(globs)]


extern crate native;
extern crate glfw;
extern crate gl;
extern crate nalgebra;

pub mod app;
pub mod engine;
pub mod shader;
pub mod tilemap;
pub mod math;


#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    // Run GLFW on the main thread
    native::start(argc, argv, main)
}

fn main() {
	app::App::run();
}
