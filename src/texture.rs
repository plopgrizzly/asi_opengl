// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::{ rc::Rc };
use OpenGL;
use types::*;

static mut CURRENT_TEXTURE: GLuint = 0; // 0 is always invalid texture.

/// An GPU Texture handle.
#[derive(Clone)] pub struct Texture(Rc<TextureContext>);

impl Texture {
	pub(crate) fn new(opengl: &OpenGL) -> Self {
		Texture(Rc::new(TextureContext({
			let mut a = unsafe { ::std::mem::uninitialized() };
			gl!(opengl, (opengl.get().gen_textures)(1, &mut a));
			gl!(opengl, (opengl.get().bind_texture)(GL_TEXTURE_2D, a));
			gl!(opengl, (opengl.get().tex_params)(GL_TEXTURE_2D,
				GL_TEXTURE_MAG_FILTER, GL_LINEAR));
			gl!(opengl, (opengl.get().tex_params)(GL_TEXTURE_2D,
				GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR)
			);
			a
		}, opengl.clone())))
	}

	/// Set the bound texture's pixels
	pub fn set(&self, w: u16, h: u16, px: &[u8]) -> () {
		self.bind();
		gl!((*self.0).1, ((*self.0).1.get().tex_image)(GL_TEXTURE_2D, 0,
			GL_RGBA as i32, w as i32, h as i32, 0, GL_RGBA,
			GL_UNSIGNED_BYTE, px.as_ptr() as *const _));
		gl!((*self.0).1, ((*self.0).1.get().gen_mipmap)(GL_TEXTURE_2D));
	}

	/// Update the pixels of an already bound & set texture.
	pub fn update(&self, w: u16, h: u16, px: &[u8]) -> () {
		self.bind();
		gl!((*self.0).1, ((*self.0).1.get().tex_subimage)(GL_TEXTURE_2D,
			0, 0, 0, w as i32, h as i32, GL_RGBA, GL_UNSIGNED_BYTE,
			px.as_ptr() as *const _));
	}

	/// Use a texture.
	pub fn bind(&self) {
		let texture = self.get();

		if texture != unsafe { CURRENT_TEXTURE } {
			gl!((*self.0).1, ((*self.0).1.get().bind_texture)(
				GL_TEXTURE_2D, texture));
			unsafe { CURRENT_TEXTURE = texture; }
		}
	}

	pub(crate) fn get(&self) -> u32 {
		(*self.0).0
	}
}

pub struct TextureContext(u32, OpenGL);

impl Drop for TextureContext {
	fn drop(&mut self) {
		gl!(self.1, (self.1.get().delete_texture)(1, [self.0].as_ptr()));
	}
}
