
extern crate gl;
use gl::types::*;

use std::ptr;
use std::str;
//use std::vec;
use std::path::Path;
use std::io::fs::File;
use std::collections::HashMap;



/*
* A shader program
* contains different shader
*/
pub struct ShaderProgram
{
	program_id: GLuint,
	linked: bool,
	shader: Vec<GLuint>,
	uniforms: HashMap<u64, GLuint>,
	attribs: HashMap<u64, GLuint>
}

impl ShaderProgram
{
	pub fn new() -> ShaderProgram
	{
		ShaderProgram {
			program_id: gl::CreateProgram(),
			linked: false,
			shader: Vec::new(),
			uniforms: HashMap::new(),
			attribs: HashMap::new()
		}
	}

	/*
	* Add a shader file with special type
	*/
	pub fn add_shader_file(&mut self, file: &str, ty:GLenum)
	{
		let path : Path   = Path::new(file);
		let mut reader : File = File::open(&path).unwrap();
		let src = reader.read_to_end().ok().unwrap();

		let shader = gl::CreateShader(ty);
		unsafe
		{
			// Attempt to compile the shader
			src.as_slice().with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
			gl::CompileShader(shader);

			// Get the compile status
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

			// Fail on error
			if status != (gl::TRUE as GLint)
			{
				let mut len = 0;
				gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::from_elem(len as uint - 1, 0u8); // subtract 1 to skip the trailing null character
				gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
				fail!("shader failed: {}", str::from_utf8(buf.as_slice()));
			}

			self.linked = false;
			self.shader.push(shader);
		}
	}

	/**
	* link together
	*/
	pub fn link_program(&mut self)
	{
		if self.linked
		{
			return;
		}

		for sid in self.shader.iter()
		{
			gl::AttachShader(self.program_id, *sid);
		}

		gl::LinkProgram(self.program_id);

		unsafe
		{
			// Get the link status
			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(self.program_id, gl::LINK_STATUS, &mut status);

			// Fail on error
			if status != (gl::TRUE as GLint)
			{
				let mut len: GLint = 0;
				gl::GetProgramiv(self.program_id, gl::INFO_LOG_LENGTH, &mut len);
				let buf = Vec::from_elem(len as uint - 1, 0u8); // subtract 1 to skip the trailing null character
				gl::GetProgramInfoLog(self.program_id, len, ptr::mut_null(), buf.as_ptr() as *mut GLchar);
				fail!("shader link fail: {}", str::from_utf8(buf.as_slice()));
			}
		}

		self.linked = true;
	}

	/*
	* Use it
	*/
	pub fn use_program(&self)
	{
		gl::UseProgram(self.program_id);
		ShaderProgram::check_gl_error();
	}

	//read in all attributes/uniforms/varying

	#[inline]
	pub fn set_fragment_name(&self, name: &str)
	{
		unsafe
		{
			//TODO error checking
			//has to be called before linking
			name.with_c_str(|ptr| gl::BindFragDataLocation(self.program_id, 0, ptr));
		}
	}

	#[inline]
	pub fn get_uniform(&self, name: &str) -> GLint
	{
		//TODO check uniforms map

		unsafe
		{
			let mut id = 0;

			name.with_c_str(|ptr| id = gl::GetUniformLocation(self.program_id, ptr));

			//id = gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8);

			if id == -1 {
				fail!("Can't retrieve Uniform: {}", name);
			}

			return id;
		}
	}

	#[inline]
	pub fn get_attrib(&self, name: &str) -> GLint
	{
		self.use_program();
		//TODO check uniforms map
		let mut id = 0;
		unsafe
		{
			name.with_c_str(|ptr| id = gl::GetAttribLocation(self.program_id, ptr));
			//id = gl::GetAttribLocation(self.program_id, name.as_ptr() as *const i8);

			if id == -1 
			{
				ShaderProgram::check_gl_error();
				fail!("Can't retrieve Attrib: \"{}\"", name);
			}
		}
		id
	}

	pub fn check_gl_error()
	{
		let gl_err = gl::GetError();
		if gl_err != gl::NO_ERROR
		{
			println!("OpenGL-Error: {}", gl_err);
		}
	}

}

impl Drop for ShaderProgram
{
  fn drop(&mut self)
  {
		// delete program
		gl::DeleteProgram(self.program_id);
		self.program_id = -1;

		//and each shader
		for sid in self.shader.iter()
		{
			gl::DeleteShader(*sid);
		}
  }
}

/*
upload matrix:
*
formated_transform = Mat4<f32>
gl::UniformMatrix4fv(context.transform, 1, gl::FALSE as u8, cast::transmute(&formated_transform)));
*/


