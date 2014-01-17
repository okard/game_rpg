

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
