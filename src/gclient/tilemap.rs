
extern crate gl;
extern crate nalgebra;

use gl::types::*;

use nalgebra::na::{Mat4};
use nalgebra::na;

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
	vbo_vertices : u32,
	vbo_indices: u32,
	vbo_tileid : u32,

	indices_count : u32

	//save model matrix

	//hold ref/owned to TilemapChunkData logical part?
	
	// tile_texture_atlas
	// tile_texture_atlas_normal? <- normal map for tiles?
}


impl TilemapChunk
{
	pub fn new() -> TilemapChunk
	{
		TilemapChunk {
			shader: shader::ShaderProgram::new(),
			vao: 0,
			vbo_vertices: 0,
			vbo_indices: 0,
			indices_count: 0,
			vbo_tileid: 0, //vbo for uv(texture) coordinates?

		}
	}
	
	// tile_count_x: how many tiles on the x axis
	// tile_count_y: how many tiles on the y axis
	pub fn setup(&mut self, tile_count_x: u32, tile_count_y: u32)
	{
		//create dummy tile layout
		let mut tilemap_chunk_vertices : Vec<GLfloat> = Vec::new();
		let mut tilemap_chunk_indices : Vec<GLuint> = Vec::new();

		//create the grid vertices

		//create tile plane vertices
		for i in range(0u32, (tile_count_x+1)*(tile_count_y+1))
		{
			let x = i % (tile_count_x+1); //first this counts up (column)
			let y = i / (tile_count_x+1); //then this counts up (row)
			
			tilemap_chunk_vertices.push(0.0+x as f32); 
			tilemap_chunk_vertices.push(0.0+y as f32); 
			
			//println!("vertex[{}]: {}, {}", i, x, y);
			
			//calculate indices for the triangles
			//indices are related to vertex indices not the vector index
				//where each vertex has 2 entries
			if x < tile_count_x
			&& y < tile_count_y
			{
				let index_of = |x :u32, y:u32| x + (y * (tile_count_x+1));
				//requires 2 triangles per tile (quad)
				tilemap_chunk_indices.push(i); //index of (x,y)
				tilemap_chunk_indices.push(index_of(x+1,y));
				tilemap_chunk_indices.push(index_of(x, y+1));
				//println!("\ttriangle_one: {}", tilemap_chunk_indices.slice_from(tilemap_chunk_indices.len()-3));
				
				tilemap_chunk_indices.push(index_of(x, y+1));
				tilemap_chunk_indices.push(index_of(x+1,y));
				tilemap_chunk_indices.push(index_of(x+1, y+1));
				//println!("\ttriangle_two: {}", tilemap_chunk_indices.slice_from(tilemap_chunk_indices.len()-3));
			}
		}
		
		self.indices_count = tilemap_chunk_indices.len() as u32;
		//println!("tilemap::setup() Count of vertices: {}", tilemap_chunk_vertices.len()/2); //x,y so /2
		//println!("tilemap::setup() Count of indices: {}", self.indices_count);
		//println!("tilemap::setup() vertices: {}", tilemap_chunk_vertices);
		
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
			gl::GenBuffers(1, &mut self.vbo_vertices);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_vertices);
			gl::BufferData(gl::ARRAY_BUFFER,
						   (tilemap_chunk_vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
						   tilemap_chunk_vertices.as_ptr() as *const GLvoid,
						   gl::STATIC_DRAW);

			// Specify the layout of the vertex data
			let vertex_attr = self.shader.get_attrib("my_vertex");
			gl::EnableVertexAttribArray(vertex_attr as GLuint);
			gl::VertexAttribPointer(vertex_attr as GLuint, 2, gl::FLOAT,
									gl::FALSE as GLboolean, 0, ptr::null());
									
			
			//vertex indices
			gl::GenBuffers(1, &mut self.vbo_indices);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo_indices);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (tilemap_chunk_indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr, 
						  tilemap_chunk_indices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

			//bind uniform

			//disable all?:
			//glBindVertexArray(0);
			//glDisableVertexAttribArray
			//gl::BindBuffer(*, 0) ? for booth?
		}
	}

	/*
	fn set_program_variable_vbo(&self, name: &str)
	{
		//in
	}
	*/

	//move to shader?
	fn set_program_uniform_mat4(&self, name: &str, m: &Mat4<f32>)
	{
		//self.shader

		let id = self.shader.get_uniform(name);
		unsafe {
			gl::UniformMatrix4fv(id, 1, gl::FALSE as u8, mem::transmute(m));
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
		//gl::DrawArrays(gl::TRIANGLES, 0, self.indices_count as i32);
		
		//with indices DrawElements must be used
		
		unsafe {
			gl::DrawElements(gl::TRIANGLE_STRIP, self.indices_count as i32, gl::UNSIGNED_INT, ptr::null());
		}

		//GL_TRIANGLE_STRIP ?

		//disable all
	}
}


