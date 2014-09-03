
extern crate glfw;
extern crate gl;

use super::engine;
use super::tilemap;
use super::math;

pub struct App
{
	window: glfw::Window,
	rc: engine::RenderContext,
	
	
}


impl App
{
	pub fn run()
	{
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		let (window, events) = glfw.create_window(1024, 768, "Game", glfw::Windowed)
			.expect("Failed to create GLFW window.");

		window.set_key_polling(true);
		(&window as &glfw::Context).make_current();
		
		// Load the OpenGL function pointers
		gl::load_with(|s| glfw.get_proc_address(s));

		//Viewport config
		let (width, height) = window.get_size();
		println!("Size: {}x{}", width, height);
		gl::Viewport(0, 0, width, height);
			
			
		//create render context
		let mut rc = engine::RenderContext::new(width as f32, height as f32);
		
		//this creates a tilemap chunk to render
		let mut tchunk = tilemap::TilemapChunk::new();
		tchunk.setup(10,10); //10x10 tiles

		//let mut v : &math::Scaling = &rc.view;
		//let mut v = &rc.view as &math::Scaling;
		//v.scale(2.0, 2.0, 1.0);
		math::scale(&mut rc.view, 50.0, 50.0, 1.0); //zoom in?
		//math::translate(&mut rc.view, -0.5, -0.5, 0.0);

		println!("projm: {}", rc.projm);
		println!("view: {}", rc.view);
		
		//gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

		let render_func = || {
			rc.draw(&tchunk);
		};
		
		
		//event loop
		while !window.should_close() 
		{
			glfw.poll_events();
			
			for (_, event) in glfw::flush_messages(&events) 
			{
				App::handle_window_event(&window, event);
			}
			
						// Clear the screen to black
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			//draw all stuff using given render callback
			render_func();

			// Swap buffers
			(&window as &glfw::Context).swap_buffers();
		}
	}
	
	/**
	* Handle Events
	*/
	fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) 
	{
		match event 
		{
			glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => 
			{
				window.set_should_close(true)
			}
			_ => {}
		}
	}
}










