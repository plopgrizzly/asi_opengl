# "asi_opengl" - Aldaron's System Interface - OpenGL
#
# Copyright Jeron A. Lau 2018.
# Distributed under the Boost Software License, Version 1.0.
# (See accompanying file LICENSE_1_0.txt or copy at
# https://www.boost.org/LICENSE_1_0.txt)

use std::{ mem, rc::Rc };
use OpenGL;
use types::*;

static mut CURRENT_BUFFER: GLuint = ::std::u32::MAX; // No current buffer

/// An OpenGL buffer, usually a VBO.
#[derive(Clone)] pub struct Buffer(pub(crate) Rc<BufferContext>);

impl Buffer {
	/// Create a new buffer
	pub fn new(opengl: &OpenGL) -> Self {
		let mut buffers = [unsafe { mem::uninitialized() }];
		gl!(opengl, (opengl.get().gen_buffers)(1/*1 buffer*/,
			buffers.as_mut_ptr()));
		Buffer(Rc::new(BufferContext(buffers[0], opengl.clone())))
	}

	/// Bind this buffer.
	pub(crate) fn bind(&self) {
		let buffer = self.get();

		if buffer != unsafe { CURRENT_BUFFER } {
			gl!((*self.0).1, ((*self.0).1.get().bind_buffer)(
				GL_ARRAY_BUFFER, buffer));
			unsafe { CURRENT_BUFFER = buffer; }
		}
	}

	/// Set the bound buffer's data
	pub fn set<T>(&self, data: &[T]) {
		self.bind();
		gl!((*self.0).1, ((*self.0).1.get().buffer_data)(
			GL_ARRAY_BUFFER,
			(data.len() * mem::size_of::<T>()) as isize,
			data.as_ptr() as *const _, GL_DYNAMIC_DRAW));
	}

	pub(crate) fn get(&self) -> GLuint {
		(*self.0).0
	}
}

pub struct BufferContext(pub(crate) GLuint, pub(crate) OpenGL);

impl Drop for BufferContext {
	fn drop(&mut self) {
		gl!(self.1, (self.1.get().delete_buffer)(1, [self.0].as_ptr()));
	}
}
