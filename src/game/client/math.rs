

use nalgebra::na::{Mat4};
use nalgebra::na;



// glOrtho(left, right, bottom, top, znear, zfar : double);

pub fn ortho_projection(left: f32, right: f32, bottom: f32, top: f32, znear: f32, zfar: f32) -> Mat4<f32>
{

	println!("ortho: l: {}, r: {}, b: {}, t: {}, n: {}, f:{}", left, right, bottom, top, znear, zfar);
	let mut m : Mat4<f32> = na::zero();

	m.m11 = 2.0/(right-left);
	m.m22 = 2.0/(top-bottom);
	m.m33 = -2.0/(zfar-znear);

	m.m14 = -((right+left)/(right-left));
	m.m24 = -((top+bottom)/(top-bottom));
	m.m34 = -((zfar+znear)/(zfar-znear));
	m.m44 = 1.0;

	return m;
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

pub fn translate(m: &mut Mat4<f32>, x: f32, y: f32, z: f32)
{
	m.m14 += x;
	m.m24 += y;
	m.m34 += z;
}


pub fn set_identity(m: &mut Mat4<f32>)
{
	m.m11 = 1.0;
	m.m21 = 0.0;
	m.m31 = 0.0;
	m.m41 = 0.0;

	m.m12 = 0.0;
	m.m22 = 1.0;
	m.m32 = 0.0;
	m.m42 = 0.0;

	m.m13 = 0.0;
	m.m23 = 0.0;
	m.m33 = 1.0;
	m.m43 = 0.0;

	m.m14 = 0.0;
	m.m24 = 0.0;
	m.m34 = 0.0;
	m.m44 = 1.0;

}

