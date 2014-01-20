
extern mod gl;
extern mod nalgebra;

use gl::types::*;

use nalgebra::na::{Mat4};
use nalgebra::na;

use std::cast;
use std::mem;
use std::ptr;

use super::engine;
use super::shader;
use super::math;

//static CHUNK_SIZE : u8 = 10;

pub struct TilemapChunk
{
	shader :shader::ShaderProgram,
	vao : u32,
	vbo_vertex : u32,
	vbo_tileid : u32,

	vertice_count : u32

	//save model matrix

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
			vbo_tileid: 0,
			vertice_count: 0
		}
	}

	pub fn setup(&mut self)
	{
		//create dummy tile layout
		let mut tiles : ~[GLfloat] = ~[];

		//create the grid vertices

		//create 5 tiles
		for t in range(0, 1)
		{
			let tf = t as f32;
			//t1 bottom left
			tiles.push(0.0+tf); //bl x
			tiles.push(0.0+tf); //bl y

			//t1 bottom right
			tiles.push(1.0+tf);
			tiles.push(0.0+tf);

			//t1 top left
			tiles.push(0.0+tf);
			tiles.push(1.0+tf);

			//t2 top left
			tiles.push(0.0+tf);
			tiles.push(1.0+tf);

			//t2 bottom right
			tiles.push(1.0+tf);
			tiles.push(0.0+tf);

			//t2 top right
			tiles.push(1.0+tf);
			tiles.push(1.0+tf);
		}
		self.vertice_count =  tiles.len() as u32/2 ; //vertice count each 2 are one point so / 2

		//create indices?

		println!("tilemap::setup() Count of vertices: {}", self.vertice_count);
		println!("tilemap::setup() vertices: {}", tiles.to_str());

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
                           (tiles.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
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


			//glBindVertexArray(0);
			//glDisableVertexAttribArray
			//gl::BindBuffer(*, 0) ?
        }
	}

	/*
	fn set_program_variable_vbo(&self, name: &str)
	{
		//in
	}
	*/


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


		let mut model : Mat4<f32> = na::zero();
		math::set_identity(&mut model);
		//set uniform
		//let mvp = /*rc.projm **/ rc.view;
		let mvp = rc.projm * rc.view * model;
		self.set_program_uniform_mat4("mvp", &mvp);

		//bind vao
		gl::BindVertexArray(self.vao);

		//render
		gl::DrawArrays(gl::TRIANGLES, 0, self.vertice_count as i32);


		/*
		glBindVertexArray(vao);
		glDrawElements(GL_TRIANGLES, NUM_INDICES, GL_UNSIGNED_BYTE, (void*)0);
		*/

		//GL_TRIANGLE_STRIP ?

		//disable all
	}
}


