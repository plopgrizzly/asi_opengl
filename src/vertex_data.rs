// "asi_opengl" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::{ ptr, cell::Cell, rc::Rc };
use types::*;
use Program;
use Buffer;

/// Vertex Data handle for a GPU Program
#[derive(Clone)] pub struct VertexData(Rc<VertexDataContext>);

impl VertexData {
	/// Get the vertex data handle for a GPU program.
	pub(crate) fn new(program: &Program, name: &[u8]) -> Self {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');
		let opengl = program.opengl();
		let attrib = gl!(opengl, (opengl.get().vdata)(program.get(),
			name.as_ptr() as *const _));
		if attrib != -1 {
			gl!(opengl, (opengl.get().enable_vdata)(attrib as u32));
		}
		VertexData(Rc::new(VertexDataContext(attrib, Cell::new(None),
			program.clone())))
	}

	/// If there is no such VertexData handle.
	pub fn is_none(&self) -> bool {
		self.0 .0 == -1
	}

	/// Set the VertexData from a Buffer
	pub fn set(&self, buffer: &Buffer) {
		let opengl = self.0 .2.opengl();
		// Hold a reference to the new buffer.
		self.0 .1.set(Some(buffer.clone()));
		// Set to the new buffer.
		buffer.bind();
		gl!(opengl, (opengl.get().vertex_attrib)(self.0 .0 as GLuint, 4,
			GL_FLOAT, 0, 0, ptr::null()));
	}
}

struct VertexDataContext(GLint/*index*/, Cell<Option<Buffer>>, Program);
