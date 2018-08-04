// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Program;
use types::*;

/// Uniform Data handle for a GPU Program
pub struct UniformData(pub(crate) GLint, Program);

impl UniformData {
	/// Get uniform from a shader.
	pub fn new(program: &Program, name: &[u8]) -> Self {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');
		let opengl = program.opengl();
		let r = gl!(opengl, (opengl.get().uniform)(program.get(),
			name.as_ptr() as *const _));
		UniformData(r, program.clone())
	}

	/// If there is no such VertexData handle.
	pub fn is_none(&self) -> bool {
		self.0 == -1
	}

	/// Set a mat4 uniform
	pub fn set_mat4(&self, mat4: [f32; 16]) -> () {
		self.1.bind(); // bind the program attached to this uniform.
		let opengl = self.1.opengl();
		gl!(opengl, (opengl.get().uniform_mat4)(self.0, 1,
			0 /*bool: transpose*/, mat4.as_ptr()));
	}

	/// Set an int uniform 
	pub fn set_int1(&self, int1: i32) -> () {
		self.1.bind(); // bind the program attached to this uniform.
		let opengl = self.1.opengl();
		gl!(opengl, (opengl.get().uniform_int1)(self.0, int1));
	}

	/// Set a float uniform
	pub fn set_vec1(&self, vec1: f32) -> () {
		self.1.bind(); // bind the program attached to this uniform.
		let opengl = self.1.opengl();
		gl!(opengl, (opengl.get().uniform_vec1)(self.0, vec1));
	}

	/// Set a vec2 uniform
	pub fn set_vec2(&self, vec: &[f32; 2]) -> () {
		self.1.bind(); // bind the program attached to this uniform.
		let opengl = self.1.opengl();
		gl!(opengl, (opengl.get().uniform_vec2)(self.0, vec[0],
			vec[1]));
	}

	/// Set a vec3 uniform
	pub fn set_vec3(&self, vec: &[f32; 3]) -> () {
		self.1.bind(); // bind the program attached to this uniform.
		let opengl = self.1.opengl();
		gl!(opengl, (opengl.get().uniform_vec3)(self.0, vec[0],
			vec[1], vec[2]));
	}

	/// Set a vec4 uniform
	pub fn set_vec4(&self, vec: &[f32; 4]) -> () {
		self.1.bind(); // bind the program attached to this uniform.
		let opengl = self.1.opengl();
		gl!(opengl, (opengl.get().uniform_vec4)(self.0, vec[0],
			vec[1], vec[2], vec[3]));
	}
}
