
extern mod gl;
extern mod nalgebra;

use gl::types::*;

use nalgebra::na::{Vec2, Mat4};
use nalgebra::na;

use std::cast;
use std::mem;

use super::engine;
use super::shader;

pub struct TilemapChunk
{
	shader :shader::ShaderProgram,
	vao : u32,
	vbo_vertex : u32,
	vbo_tileid : u32
}


impl TilemapChunk
{
	pub fn new() -> TilemapChunk
	{
		TilemapChunk {
			shader: shader::ShaderProgram::new(),
			vao: 0,
			vbo_vertex: 0,
			vbo_tileid: 0
		}
	}

	pub fn setup(&mut self)
	{
		//create dummy tile layout
		let mut tiles : ~[Vec2<GLfloat>] = ~[];

		//create the grid vertices
		for x in range(0, 100)
		{
			for y in range(0, 100)
			{
				tiles.push(Vec2::new(x as GLfloat, y as GLfloat));
			}
		}

		//TODO shader config elsewhere?
		self.shader.add_shader_file("./data/client/shader/tilemap.vs.glsl", gl::VERTEX_SHADER);
		self.shader.add_shader_file("./data/client/shader/tilemap.fs.glsl", gl::FRAGMENT_SHADER);
		self.shader.set_fragment_name("out_color");
		self.shader.link_program();

        unsafe
        {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut self.vbo_vertex);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_vertex);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (tiles.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           cast::transmute(&tiles[0]),
                           gl::STATIC_DRAW);

            // Specify the layout of the vertex data
            //let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
            //gl::EnableVertexAttribArray(pos_attr as GLuint);
            //gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
            //                        gl::FALSE as GLboolean, 0, ptr::null());


            //create vbo tile_ids
			//bind vbo
			//fill vbo

			//bind uniform
        }
	}

	fn set_program_variable_vbo(&self, name: &str)
	{
		//in
	}


	fn set_program_uniform_mat4(&self, name: &str, m: &Mat4<f32>)
	{
		//self.shader

		let id = self.shader.get_uniform(name);
		unsafe {
			gl::UniformMatrix4fv(id, 1, gl::FALSE as u8, cast::transmute(m));
		}
	}
}

impl engine::Drawable for TilemapChunk
{
	fn draw(&self, rc: &engine::RenderContext)
	{
		//use shader
		self.shader.use_program();

		//bind vao

		//set uniform
			//matrices

		//render

		//disable all

	}
}


