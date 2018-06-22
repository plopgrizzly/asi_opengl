# "asi_opengl" - Aldaron's System Interface - OpenGL
#
# Copyright Jeron A. Lau 2018.
# Distributed under the Boost Software License, Version 1.0.
# (See accompanying file LICENSE_1_0.txt or copy at
# https://www.boost.org/LICENSE_1_0.txt)

use UniformData;
use VertexData;
use OpenGL;
use std::{ rc::Rc, ops::Range };
use types::*;
use Topology;

static mut CURRENT_PROGRAM: GLuint = 0; // 0 is always invalid program.

/// A loaded GPU program.
#[derive(Clone)] pub struct Program(pub(crate) Rc<ProgramContext>);

impl Program {
	/// Load a shader program
	pub fn new(opengl: &OpenGL, vertex: &[u8], fragment: &[u8]) -> Self {
		// Compile vertex & fragment shaders
		let v_shader = shader_new(opengl, 0x8B31/*vertex*/, vertex);
		let f_shader = shader_new(opengl, 0x8B30/*fragment*/, fragment);
		// Link shaders together.
		let program = gl!(opengl, (opengl.get().create_program)());
		gl!(opengl, (opengl.get().attach_shader)(program, v_shader));
		gl!(opengl, (opengl.get().attach_shader)(program, f_shader));
		gl!(opengl, (opengl.get().link_program)(program));
		gl!(opengl, (opengl.get().detach_shader)(program, v_shader));
		gl!(opengl, (opengl.get().detach_shader)(program, f_shader));
		// Return
		Program(Rc::new(ProgramContext(program, opengl.clone())))
	}

	/// Get a vertex data handle for this GPU program.
	pub fn vertex_data(&self, name: &[u8]) -> VertexData {
		VertexData::new(self, name)
	}

	/// Get a uniform data handle for this GPU program.
	pub fn uniform(&self, name: &[u8]) -> UniformData {
		UniformData::new(self, name)
	}

	/// Draw the elements.
	pub fn draw_arrays(&self, topology: Topology, range: Range<u32>) {
		self.bind();
		gl!((*self.0).1, ((*self.0).1.get().draw_arrays)(
			topology as GLuint,
			range.start as GLint, range.end as GLsizei));
	}

	/// Bind a program to be used.
	pub(crate) fn bind(&self) {
		let program = unsafe { self.get() };

		if program != unsafe { CURRENT_PROGRAM } {
			gl!(&(*self.0).1,
				((*self.0).1.get().use_program)(program));
			unsafe { CURRENT_PROGRAM = program; }
		}
	}

	/// Get a new OpenGL reference
	pub(crate) fn opengl(&self) -> OpenGL {
		(*self.0).1.clone()
	}

	pub(crate) unsafe fn get(&self) -> GLuint {
		(*self.0).0
	}
}

pub(crate) struct ProgramContext(GLuint, OpenGL);

impl Drop for ProgramContext {
	fn drop(&mut self) {
		gl!(self.1, (self.1.get().delete_program)(self.0));
	}
}

/// Compile a new shader.
fn shader_new(opengl: &OpenGL, shader_type: GLenum, src: &[u8]) -> GLuint {
	let shader = gl!(opengl, (opengl.get().create_shader)(shader_type));
	gl!(opengl, (opengl.get().shader_source)(shader, 1 /*1 string*/,
		[src.as_ptr() as *const _].as_ptr(), [src.len() as i32].as_ptr()
	));
	gl!(opengl, (opengl.get().compile_shader)(shader));
	compile_errors(opengl, shader);
	shader
}

/// Evaluate and panic with error message if failed to compile, does nothing in
/// release mode
fn compile_errors(_opengl: &OpenGL, _shader: GLuint) {
	#[cfg(debug_assertions)] {
		let mut value = unsafe { ::std::mem::uninitialized() };

		gl!(_opengl, (_opengl.get().get_shader)(_shader,
			0x8B81 /*GL_COMPILE_STATUS*/, &mut value));

		if value == 0 {
			let mut value = unsafe { ::std::mem::uninitialized() };
			gl!(_opengl, (_opengl.get().get_shader)(_shader,
				0x8B84 /*GL_INFO_LOG_LENGTH*/,
				&mut value));
			let mut buffer: Vec<u8> =
				vec![unsafe { ::std::mem::uninitialized() };
					value as usize];
			gl!(_opengl, (_opengl.get().info_log)(_shader,
				value as GLsizei, ::std::ptr::null_mut(),
				buffer.as_mut_ptr() as *mut _));
			panic!("Failed to compile: {}.",
				::std::str::from_utf8(buffer.as_slice())
					.unwrap());
		}
	}
}
