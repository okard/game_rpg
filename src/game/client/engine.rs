
extern mod nalgebra;
use nalgebra::na::{Mat4};
use nalgebra::na;


pub struct RenderContext //is trait?
{
	//view
	//shaderprogram
	//texture
	//size

	// handle mvp -> model-view-projection matrix
	// save view (Camera)
	// save projection
	view: Mat4<f32>
}

/*
* Function for Render Context
*/
impl RenderContext
{
	pub fn new() -> RenderContext
	{
		RenderContext { view: na::zero() }
	}

	//shader switch
	//texture switch

	pub fn draw(&self, drawable: &Drawable)
	{
		drawable.draw(self);
	}
}


/*
upload matrix:
*
formated_transform = Mat4<f32>
gl::UniformMatrix4fv(context.transform, 1, gl::FALSE as u8, cast::transmute(&formated_transform)));
*/


pub trait Drawable
{
	fn draw(&self, rc: &RenderContext);
}
