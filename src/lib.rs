// "asi_opengl" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

#[macro_use]
extern crate dl_api;
#[cfg(windows)]
extern crate winapi;

macro_rules! gl {
	($o: expr, $a: stmt) => (
		unsafe {
			let a = { $a };
			$o.error();
			a
		}
	)
}

use std::os::raw::c_void;
use std::rc::Rc;
use std::cell::RefCell;

mod loader;
mod types;

use types::*;

mod texture;
mod vertex_data;
mod uniform_data;
mod program;
mod buffer;

pub use vertex_data::VertexData;
pub use uniform_data::UniformData;
pub use program::Program;
pub use buffer::Buffer;
pub use texture::Texture;

/// Features that can be enabled and disabled.
#[repr(u32)]
pub enum Feature {
	Dither = 0x0BD0,
	CullFace = 0x0B44,
	Blend = 0x0BE2,
	DepthTest = 0x0B71,
}

/// What the vertices represent
#[repr(u32)]
pub enum Topology {
	Points = 0x0000,
	Lines = 0x0001,
	LineLoop = 0x0002,
	LineStrip = 0x0003,
	Triangles = 0x0004,
	TriangleStrip = 0x0005,
	TriangleFan = 0x0006,
}

/// The OpenGL context.
#[derive(Clone)] pub struct OpenGL(Rc<RefCell<OpenGLContext>>);

/// The OpenGL builder.
pub struct OpenGLBuilder {
	lib: loader::Lib,
	display: loader::Display,
}

impl OpenGLBuilder {
	/// Begin the building.
	pub fn new() -> Option<(OpenGLBuilder, i32)> {
		if let Ok(lib) = loader::Lib::new() {
			let (mut display, visual_id) = lib.init();

			Some((OpenGLBuilder {
				lib,
				display,
			}, visual_id))
		} else {
			None
		}
	}

	/// Complete the building
	pub fn to_opengl(mut self, window: EGLNativeWindowType) -> OpenGL {
		self.lib.init2(&mut self.display, window);

		OpenGL(Rc::new(RefCell::new(OpenGLContext {
			// FFI OpenGL Functions.
			clear: self.lib.load(b"glClear\0"),
			clear_color: self.lib.load(b"glClearColor\0"),
			disable: self.lib.load(b"glDisable\0"),
			enable: self.lib.load(b"glEnable\0"),
			#[cfg(debug_assertions)]
			get_error: self.lib.load(b"glGetError\0"),
			blend_func_separate:
				self.lib.load(b"glBlendFuncSeparate\0"),
			create_shader: self.lib.load(b"glCreateShader\0"),
			shader_source: self.lib.load(b"glShaderSource\0"),
			compile_shader: self.lib.load(b"glCompileShader\0"),
			create_program: self.lib.load(b"glCreateProgram\0"),
			attach_shader: self.lib.load(b"glAttachShader\0"),
			link_program: self.lib.load(b"glLinkProgram\0"),
			uniform: self.lib.load(b"glGetUniformLocation\0"),
			gen_buffers: self.lib.load(b"glGenBuffers\0"),
			bind_buffer: self.lib.load(b"glBindBuffer\0"),
			buffer_data: self.lib.load(b"glBufferData\0"),
			vdata: self.lib.load(b"glGetAttribLocation\0"),
			#[cfg(debug_assertions)]
			get_shader: self.lib.load(b"glGetShaderiv\0"),
			#[cfg(debug_assertions)]
			info_log: self.lib.load(b"glGetShaderInfoLog\0"),
			draw_arrays: self.lib.load(b"glDrawArrays\0"),
			use_program: self.lib.load(b"glUseProgram\0"),
			uniform_mat4: self.lib.load(b"glUniformMatrix4fv\0"),
			uniform_int1: self.lib.load(b"glUniform1i\0"),
			uniform_vec1: self.lib.load(b"glUniform1f\0"),
			uniform_vec2: self.lib.load(b"glUniform2f\0"),
			uniform_vec3: self.lib.load(b"glUniform3f\0"),
			uniform_vec4: self.lib.load(b"glUniform4f\0"),
			bind_texture: self.lib.load(b"glBindTexture\0"),
			vertex_attrib: self.lib.load(b"glVertexAttribPointer\0"),
			gen_textures: self.lib.load(b"glGenTextures\0"),
			tex_params: self.lib.load(b"glTexParameteri\0"),
			tex_image: self.lib.load(b"glTexImage2D\0"),
			tex_subimage: self.lib.load(b"glTexSubImage2D\0"),
			enable_vdata: self.lib.load(b"glEnableVertexAttribArray\0"),
			viewport: self.lib.load(b"glViewport\0"),
			gen_mipmap: self.lib.load(b"glGenerateMipmap\0"),
			detach_shader: self.lib.load(b"glDetachShader\0"),
			delete_program: self.lib.load(b"glDeleteProgram\0"),
			delete_buffer: self.lib.load(b"glDeleteBuffers\0"),
			delete_texture: self.lib.load(b"glDeleteTextures\0"),
			// Other
			display: self.display,
			lib: self.lib,
		})))
	}
}

/// The OpenGL context.
struct OpenGLContext {
	#[allow(unused)] // is used at drop.
	lib: loader::Lib,
	display: loader::Display,
	clear: unsafe extern "system" fn(GLbitfield) -> (),
	clear_color: unsafe extern "system" fn(GLfloat, GLfloat, GLfloat,
		GLfloat) -> (),
	disable: unsafe extern "system" fn(GLenum) -> (),
	enable: unsafe extern "system" fn(GLenum) -> (),
	#[cfg(debug_assertions)] get_error: unsafe extern "system" fn() -> GLenum,
	blend_func_separate: unsafe extern "system" fn(GLenum, GLenum, GLenum,
		GLenum) -> (),
	create_shader: unsafe extern "system" fn(GLenum) -> GLuint,
	shader_source: unsafe extern "system" fn(GLuint, GLsizei,
		*const *const GLchar, *const GLint) -> (),
	compile_shader: unsafe extern "system" fn(GLuint) -> (),
	create_program: unsafe extern "system" fn() -> GLuint,
	attach_shader: unsafe extern "system" fn(GLuint, GLuint) -> (),
	link_program: unsafe extern "system" fn(GLuint) -> (),
	uniform: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint,
	gen_buffers: unsafe extern "system" fn(GLsizei, *mut GLuint) -> (),
	bind_buffer: unsafe extern "system" fn(GLenum, GLuint) -> (),
	buffer_data: unsafe extern "system" fn(GLenum, GLsizeiptr,
		*const c_void, GLenum) -> (),
	vdata: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint,
	#[cfg(debug_assertions)]
	get_shader: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> (),
	#[cfg(debug_assertions)]
	info_log: unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei,
		*mut GLchar) -> (),
	draw_arrays: unsafe extern "system" fn(GLenum, GLint, GLsizei) -> (),
	use_program: unsafe extern "system" fn(GLuint) -> (),
	uniform_mat4: unsafe extern "system" fn(GLint, GLsizei, GLboolean,
		*const GLfloat) -> (),
	uniform_int1: unsafe extern "system" fn(GLint, GLint) -> (),
	uniform_vec1: unsafe extern "system" fn(GLint, GLfloat) -> (),
	uniform_vec2: unsafe extern "system" fn(GLint, GLfloat, GLfloat) -> (),
	uniform_vec3: unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat)
		-> (),
	uniform_vec4: unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat,
		GLfloat) -> (),
	bind_texture: unsafe extern "system" fn(GLenum, GLuint) -> (),
	vertex_attrib: unsafe extern "system" fn(GLuint, GLint, GLenum,
		GLboolean, GLsizei, *const c_void) -> (),
	gen_textures: unsafe extern "system" fn(GLsizei, *mut GLuint) -> (),
	tex_params: unsafe extern "system" fn(GLenum, GLenum, GLint) -> (),
	tex_image: unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei,
		GLsizei, GLint, GLenum, GLenum, *const c_void) -> (),
	tex_subimage: unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei,
		GLsizei, GLenum, GLenum, *const c_void) -> (),
	enable_vdata: unsafe extern "system" fn(GLuint) -> (),
	viewport: unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> (),
	gen_mipmap: unsafe extern "system" fn(GLenum) -> (),
	detach_shader: unsafe extern "system" fn(GLuint, GLuint) -> (),
	delete_program: unsafe extern "system" fn(GLuint) -> (),
	delete_buffer: unsafe extern "system" fn(GLsizei, *const GLuint) -> (),
	delete_texture: unsafe extern "system" fn(GLsizei, *const GLuint) -> (),
}

impl OpenGL {
	/// Set the color for `clear`.
	pub fn color(&self, r: f32, g: f32, b: f32) {
		gl!(self, (self.get().clear_color)(r, g, b, 1.0));
	}

	/// Update the screen
	pub fn update(&self) {
		// Swap Display
		self.get().display.swap(
			#[cfg(not(target_os = "windows"))]
			&self.get().lib
		);
		// Clear Color & Depth
		gl!(self, (self.get().clear)(0x00000100 | 0x00004000));
	}

	/// Enable something
	pub fn enable(&self, what: Feature) {
		gl!(self, (self.get().enable)(what as u32))
	}

	/// Disable something
	pub fn disable(&self, what: Feature) {
		gl!(self, (self.get().disable)(what as u32))
	}

	/// Configure blending
	pub fn blend(&self) {
		const GL_SRC_ALPHA: u32 = 0x0302;
		const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
		const GL_DST_ALPHA: u32 = 0x0304;

		gl!(self, (self.get().blend_func_separate)(
			GL_SRC_ALPHA,
			GL_ONE_MINUS_SRC_ALPHA,
			GL_SRC_ALPHA,
			GL_DST_ALPHA
		));
	}

	/// Create a new texture.
	pub fn texture(&self) -> Texture {
		Texture::new(self)
	}

	/// Update the viewport.
	pub fn viewport(&self, w: u32, h: u32) {
		gl!(self, (self.get().viewport)(0, 0, w as GLsizei,
			h as GLsizei));
	}

	#[cfg(not(debug_assertions))]
	unsafe fn error(&self) { /* Do nothing in release mode for speed. */ }

	#[cfg(debug_assertions)]
	unsafe fn error(&self) {
		match (self.get().get_error)() {
			0 => return, // NO_ERROR
			0x0500 => panic!("OpenGL Error: Invalid enum"),
			0x0501 => panic!("OpenGL Error: Invalid value"),
			0x0502 => panic!("OpenGL Error: Invalid operation"),
			0x0503 => panic!("OpenGL Error: Stack overflow"),
			0x0504 => panic!("OpenGL Error: Stack underflow"),
			0x0505 => panic!("OpenGL Error: Out of memory"),
			_ => panic!("OpenGL Error: Unknown"),
		}
	}

	fn get(&self) -> std::cell::Ref<OpenGLContext> {
		self.0.borrow()
	}
}
