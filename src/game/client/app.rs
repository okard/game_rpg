
extern mod glfw;
extern mod gl;


mod engine;
mod shader;

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

		// Set up an error callback
		glfw::set_error_callback(~ErrorContext);

		// Initialize the library
		do glfw::start
		{
			// Create a windowed mode window and its OpenGL context
			let window = glfw::Window::create(800, 600, "Game", glfw::Windowed)
				.expect("Failed to create GLFW window.");

			// Make the window's context current
			window.make_context_current();

			// Load the OpenGL function pointers
			gl::load_with(glfw::get_proc_address);

			let mut sp = shader::ShaderProgram::new();

			sp.add_shader_file("./data/client/shader/default.vs", gl::VERTEX_SHADER);
			sp.add_shader_file("./data/client/shader/default.fs", gl::FRAGMENT_SHADER);
			sp.link_program();

			//Run event loop
			App::run_event_loop(&window);
		}
	}

	fn run_event_loop(window : &glfw::Window)
	{
		// Loop until the user closes the window
		while !window.should_close()
		{
			// Swap front and back buffers
			window.swap_buffers();

			// Poll for and process events
			glfw::poll_events();
		}
	}
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}









