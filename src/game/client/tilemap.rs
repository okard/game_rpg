
extern mod gl;
extern mod nalgebra;

use gl::types::*;

use nalgebra::na::{Vec2, Mat4};
use nalgebra::na;

use std::cast;
use std::mem;
use std::ptr;

use super::engine;
use super::shader;

static CHUNK_SIZE : u8 = 5;

pub struct TilemapChunk
{
	shader :shader::ShaderProgram,
	vao : u32,
	vbo_vertex : u32,
	vbo_tileid : u32

	//hold ref/owned to TilemapChunkData logical part?
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
		let mut tiles : ~[GLfloat] = ~[];

		//create the grid vertices
		for x in range(0, CHUNK_SIZE)
		{
			for y in range(0, CHUNK_SIZE)
			{
				tiles.push(x as GLfloat);
				tiles.push(y as GLfloat);
			}
		}

		println!("tilemap::setup() Count of vertices: {}", tiles.len());

		//TODO shader config elsewhere?
		self.shader.add_shader_file("./data/client/shader/tilemap.vs.glsl", gl::VERTEX_SHADER);
		self.shader.add_shader_file("./data/client/shader/tilemap.fs.glsl", gl::FRAGMENT_SHADER);
		self.shader.set_fragment_name("fragColor"); //required before linking
		self.shader.link_program();
		self.shader.use_program();

        unsafe
        {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut self.vbo_vertex);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_vertex);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (tiles.len()/2 * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           cast::transmute(&tiles[0]),
                           gl::STATIC_DRAW);

            // Specify the layout of the vertex data
            let vertex_attr = self.shader.get_attrib("vertex");
            gl::EnableVertexAttribArray(vertex_attr as GLuint);
            gl::VertexAttribPointer(vertex_attr as GLuint, 2, gl::FLOAT,
                                    gl::FALSE as GLboolean, 0, ptr::null());


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
		unsafe { gl::BindVertexArray(self.vao); }

		//set uniform
		self.set_program_uniform_mat4("view", &rc.view);
		//self.set_program_uniform_mat4("projm", &rc.projm);

		//render
		let triangle_count = CHUNK_SIZE as i32 * CHUNK_SIZE as i32;
		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, triangle_count);

		//GL_TRIANGLE_STRIP ?

		//disable all



	}
}


