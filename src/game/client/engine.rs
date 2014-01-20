
extern mod nalgebra;
use nalgebra::na::{Mat4};
use nalgebra::na;

use super::math;


pub struct RenderContext //is trait?
{
	//view
	//shaderprogram
	//texture
	//size

	// handle mvp -> model-view-projection matrix
	// save view (Camera)
	// save projection
	view: Mat4<f32>,
	projm: Mat4<f32>
}

//split? -> GlContext (projection matrix) -> RenderContext (view matrix) -> Mesh/Sprite (model matrix)

/*
* Function for Render Context
*/
impl RenderContext
{
	pub fn new() -> RenderContext
	{
		RenderContext { view: math::simple_view(), projm: math::ortho_projection(0.0, 0.0, 100.0, 100.0, -5.0, 5.0) }
	}

	//shader switch
	//texture switch

	pub fn draw(&self, drawable: &Drawable)
	{
		drawable.draw(self);
	}
}


pub trait Drawable
{
	fn draw(&self, rc: &RenderContext);
}
