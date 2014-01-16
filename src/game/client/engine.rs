

mod shader;

pub struct RenderContext //is trait?
{
	dummy: int

	//view
	//shaderprogram
	//texture
	//size
}

impl RenderContext
{
	//shader switch
	//texture switch

	pub fn draw(&self, drawable: &Drawable)
	{
		drawable.draw(self);
	}
}


trait Drawable
{
	fn draw(&self, rc: &RenderContext);
}
