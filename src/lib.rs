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
use std::{ mem };
use std::rc::Rc;
use std::cell::RefCell;

mod loader;
mod types;

use types::*;

mod vertex_data;
mod program;
mod buffer;

pub use vertex_data::VertexData;
pub use program::Program;
pub use buffer::Buffer;

/// An GPU Texture handle.
#[derive(Copy, Clone)] pub struct Texture(u32);

#[derive(Clone)] pub struct OpenGL(Rc<RefCell<OpenGLContext>>);
pub struct Shader(Rc<(OpenGL, u32)>);

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
			// Other
			display: self.display,
			lib: self.lib,
		})))
	}
}

/// The OpenGL context.
pub struct OpenGLContext {
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

	// TODO: what should be an enum for safety.
	/// Enable something
	pub fn enable(&self, what: u32) {
		gl!(self, (self.get().enable)(what))
	}

	// TODO: what should be an enum for safety.
	/// Disable something
	pub fn disable(&self, what: u32) {
		gl!(self, (self.get().disable)(what))
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

	/// Get uniform from a shader.
	pub fn uniform(&self, program: &Program, name: &[u8]) -> i32 {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');
		let r = gl!(self, (self.get().uniform)(program.get(),
			name.as_ptr() as *const _));
		if r == -1 {
			panic!("Error No Uniform: {:?}",
				::std::str::from_utf8(name).unwrap());
		}
		r
	}

	/// Bind a buffer from `new_buffers()`
	pub fn bind_buffer(&self, buffer: &Buffer) {
		// TODO: Check if it's already bound first.
		gl!(self, (self.get().bind_buffer)(GL_ARRAY_BUFFER,
			buffer.get()));
	}

	/// Set the bound buffer's data
	pub fn set_buffer<T>(&self, data: &[T]) {
		gl!(self, (self.get().buffer_data)(GL_ARRAY_BUFFER,
			(data.len() * mem::size_of::<T>()) as isize,
			data.as_ptr() as *const _, GL_DYNAMIC_DRAW));
	}

	// TODO: this actually unsafe because uniforms can only be accessed when
	// their program is in use.
	/// Use a program.
	pub fn use_program(&self, program: &Program) {
		gl!(self, (self.get().use_program)(program.get()));
	}

	/// Set a mat4 uniform
	pub fn set_mat4(&self, uniform: i32, mat4: &[f32; 16]) -> () {
		gl!(self, (self.get().uniform_mat4)(uniform, 1,
			0 /*bool: transpose*/, mat4.as_ptr()));
	}

	/// Set an int uniform 
	pub fn set_int1(&self, uniform: i32, int1: i32) -> () {
		gl!(self, (self.get().uniform_int1)(uniform, int1));
	}

	/// Set a float uniform
	pub fn set_vec1(&self, uniform: i32, vec1: f32) -> () {
		gl!(self, (self.get().uniform_vec1)(uniform, vec1));
	}

	/// Set a vec2 uniform
	pub fn set_vec2(&self, uniform: i32, vec: &[f32; 2]) -> () {
		gl!(self, (self.get().uniform_vec2)(uniform, vec[0], vec[1]));
	}

	/// Set a vec3 uniform
	pub fn set_vec3(&self, uniform: i32, vec: &[f32; 3]) -> () {
		gl!(self, (self.get().uniform_vec3)(uniform, vec[0], vec[1],
			vec[2]));
	}

	/// Set a vec4 uniform
	pub fn set_vec4(&self, uniform: i32, vec: &[f32; 4]) -> () {
		gl!(self, (self.get().uniform_vec4)(uniform, vec[0], vec[1],
			vec[2], vec[3]));
	}

	/// Draw the elements.
	pub fn draw_arrays(&self, start_index: u32, n_indices: u32) {
		gl!(self, (self.get().draw_arrays)(0x0006 /*GL_TRIANGLE_FAN*/,
			start_index as GLint, n_indices as GLsizei));
	}

	/// Create a new texture.
	pub fn new_texture(&self) -> Texture {
		Texture({
			let mut a = unsafe { mem::uninitialized() };
			gl!(self, (self.get().gen_textures)(1, &mut a));
			self.use_texture(&Texture(a));
			gl!(self, (self.get().tex_params)(GL_TEXTURE_2D,
				GL_TEXTURE_MAG_FILTER, GL_LINEAR));
			gl!(self, (self.get().tex_params)(GL_TEXTURE_2D,
				GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR)
			);
			a
		})
	}

	/// Set the bound texture's pixels
	pub fn set_texture(&self, w: u32, h: u32, px: &[u32]) -> () {
		gl!(self, (self.get().tex_image)(GL_TEXTURE_2D, 0,
			GL_RGBA as i32, w as i32, h as i32, 0, GL_RGBA,
			GL_UNSIGNED_BYTE, px.as_ptr() as *const _));
		gl!(self, (self.get().gen_mipmap)(GL_TEXTURE_2D));
	}

	/// Update the pixels of an already bound & set texture.
	pub fn texture_update(&self, w: u32, h: u32, px: &[u32]) -> () {
		gl!(self, (self.get().tex_subimage)(GL_TEXTURE_2D, 0, 0, 0,
			w as i32, h as i32, GL_RGBA, GL_UNSIGNED_BYTE,
			px.as_ptr() as *const _));
	}

	/// Use a texture.
	pub fn use_texture(&self, texture: &Texture) {
		gl!(self, (self.get().bind_texture)(GL_TEXTURE_2D, texture.0));
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
