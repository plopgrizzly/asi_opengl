// "asi_opengl" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::{ mem, rc::Rc };
use OpenGL;
use types::*;

#[derive(Clone)] pub struct Buffer(pub(crate) Rc<BufferContext>);

impl Buffer {
	/// Create a new buffer
	pub fn new(opengl: &OpenGL) -> Self {
		let mut buffers = [unsafe { mem::uninitialized() }];
		gl!(opengl, (opengl.get().gen_buffers)(1/*1 buffer*/,
			buffers.as_mut_ptr()));
		Buffer(Rc::new(BufferContext(buffers[0], opengl.clone())))
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
