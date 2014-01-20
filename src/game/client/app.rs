
extern mod glfw;
extern mod gl;

use super::engine;
use super::tilemap;
use super::math;

pub struct App
{
	window: glfw::Window,
	rc: engine::RenderContext
}


impl App
{
	pub fn run()
	{
		// Choose a GL profile that is compatible with OS X 10.7+
		glfw::window_hint::context_version(3, 2);
		glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
		glfw::window_hint::opengl_forward_compat(true);

		//TODO remove when handled
		glfw::window_hint::resizable(false);

		// Set up an error callback
		glfw::set_error_callback(~ErrorContext);

		// Initialize the library
		do glfw::start
		{
			// Create a windowed mode window and its OpenGL context
			let window = glfw::Window::create(1024, 768, "Game", glfw::Windowed)
				.expect("Failed to create GLFW window.");

			window.set_size_callback(~WindowSizeContext);

			// Make the window's context current
			window.make_context_current();

			// Load the OpenGL function pointers
			gl::load_with(glfw::get_proc_address);

			//Viewport config
			let (width, height) = window.get_size();
			println!("Size: {}x{}", width, height);
			gl::Viewport(0, 0, width, height);

			//create render context
			let mut rc = engine::RenderContext::new(width as f32, height as f32);
			let mut tchunk = tilemap::TilemapChunk::new();
			tchunk.setup();

			//let mut v : &math::Scaling = &rc.view;
			//let mut v = &rc.view as &math::Scaling;
			//v.scale(2.0, 2.0, 1.0);
			math::scale(&mut rc.view, 200.0, 200.0, 1.0);
			//math::translate(&mut rc.view, -0.5, -0.5, 0.0);

			println!("projm: {}", rc.projm.to_str());
			println!("view: {}", rc.view.to_str());

			gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);



			let render_func = || {
				rc.draw(&tchunk);
			};

			//Run event loop
			App::run_event_loop(&window, render_func);
		}
	}

	fn run_event_loop(window : &glfw::Window, rf: ||)
	{
		// Loop until the user closes the window
		while !window.should_close()
		{
			// Poll events
			glfw::poll_events();

			// Clear the screen to black
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			//draw all stuff
			rf();

			// Swap buffers
			window.swap_buffers();
		}
	}
}

struct WindowSizeContext;
impl glfw::WindowSizeCallback for WindowSizeContext
{
    fn call(&self, window: &glfw::Window, width: i32, height: i32)
    {
        println!("Window size: ({}, {})", width, height);
        gl::Viewport(0, 0, width, height);
        //TODO update projection matrix here
    }
}


struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
	fn call(&self, _: glfw::Error, description: ~str) {
		println!("GLFW Error: {:s}", description);
	}
}









