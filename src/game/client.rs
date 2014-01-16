
#[feature(globs)];


extern mod native;
extern mod glfw;
extern mod gl;

mod client {
	pub mod app;
}


#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread
    native::start(argc, argv, main)
}

fn main() {
	client::app::App::run();
}
