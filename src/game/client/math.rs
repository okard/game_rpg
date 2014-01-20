

use nalgebra::na::{Mat4};


// glOrtho(left, right, bottom, top, znear, zfar : double);

pub fn ortho_projection(left: f32, bottom: f32, right: f32, top: f32, znear: f32, zfar: f32) -> Mat4<f32>
{
	//column wise
	Mat4::new(	2.0/(right-left), 0.0, 0.0, 0.0,
				0.0, 2.0/(top-bottom), 0.0, 0.0,
				0.0, 0.0, -2.0/(zfar-znear), 0.0,
				-((right+left)/(right-left)), -((top+bottom)/(top-bottom)), -((zfar+znear)/(zfar-znear)), 1.0)
}

pub fn simple_view() -> Mat4<f32>
{
	Mat4::new(	1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, 0.0,
				0.0, 0.0, 0.0, 1.0)
}



pub trait Scaling
{
	fn scale(&mut self, x: f32, y: f32, z: f32);

	//fn scale_set
	// fn
}

/*

m11 m12 m13 m14
m21 m22 m23 m24
m31 m32 m33 m34
m41 m42 m43 m44
*/

impl Scaling for Mat4<f32>
{
	fn scale(&mut self, x: f32, y: f32, z: f32)
	{
		self.m11 *= x;
		self.m22 *= y;
		self.m33 *= z;
	}
}


pub fn scale(m: &mut Mat4<f32>, x: f32, y: f32, z: f32)
{
	m.m11 *= x;
	m.m22 *= y;
	m.m33 *= z;
}


