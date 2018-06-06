// "asi_opengl" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

#[macro_use]
extern crate dl_api;
#[cfg(windows)]
extern crate winapi;

use std::os::raw::c_void;
use std::{ mem, ptr };

mod loader;
mod types;

use types::*;

#[derive(Copy, Clone)]
pub struct Texture(u32);
pub struct Attribute(pub GLint); // Pub is for testing,

/// The OpenGL builder.
pub struct OpenGLBuilder {
	lib: loader::Lib,
	display: loader::Display,
}

impl OpenGLBuilder {
	/// Begin the building.
	pub fn new() -> Option<(OpenGLBuilder, i32)> {
		if let Some(lib) = loader::Lib::new() {
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

		OpenGL {
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
			attribute: self.lib.load(b"glGetAttribLocation\0"),
			get_shader: self.lib.load(b"glGetShaderiv\0"),
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
			enable_vattrib: self.lib.load(b"glEnableVertexAttribArray\0"),
			viewport: self.lib.load(b"glViewport\0"),
			gen_mipmap: self.lib.load(b"glGenerateMipmap\0"),
			// Other
			display: self.display,
			lib: self.lib,
		}
	}
}

/// The OpenGL context.
pub struct OpenGL {
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
	attribute: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint,
	get_shader: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> (),
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
	enable_vattrib: unsafe extern "system" fn(GLuint) -> (),
	viewport: unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> (),
	gen_mipmap: unsafe extern "system" fn(GLenum) -> (),
}

impl OpenGL {
	/// Clear the screen with a specific color.
	pub fn clear(&self) {
		// Clear Color & Depth
		unsafe {
			(self.clear)(0x00000100 | 0x00004000);
			self.error()
		}
	}

	/// Set the color for `clear`.
	pub fn color(&self, r: f32, g: f32, b: f32) {
		unsafe {
			(self.clear_color)(r, g, b, 1.0);
			self.error()
		}
	}

	/// Update the screen
	pub fn update(&self) {
		// Swap Display
		self.display.swap(
			#[cfg(not(target_os = "windows"))]
			&self.lib
		);
	}

	/// Enable something
	pub fn enable(&self, what: u32) {
		unsafe {
			(self.enable)(what);
			self.error()
		}
	}

	/// Disable something
	pub fn disable(&self, what: u32) {
		unsafe {
			(self.disable)(what);
			self.error()
		}
	}

	/// Configure blending
	pub fn blend(&self) {
		const GL_SRC_ALPHA : u32 = 0x0302;
		const GL_ONE_MINUS_SRC_ALPHA : u32 = 0x0303;
		const GL_DST_ALPHA : u32 = 0x0304;

		unsafe {
			(self.blend_func_separate)(
				GL_SRC_ALPHA,
				GL_ONE_MINUS_SRC_ALPHA,
				GL_SRC_ALPHA,
				GL_DST_ALPHA
			);
			self.error()
		}
	}

	/// Load a shader program
	pub fn shader(&self, vertex: &[u8], fragment: &[u8]) -> u32 {
		// Last character in slices needs to null for it to be safe.
		assert_eq!(vertex[vertex.len() -1], b'\0');
		assert_eq!(fragment[fragment.len() -1], b'\0');

		let program;

		unsafe {
			self.error();
			let v_shader = (self.create_shader)(0x8B31/*vertex*/);
			self.error();
			(self.shader_source)(v_shader, 1,
				[vertex.as_ptr() as *const _].as_ptr(), ptr::null());
			self.error();
			(self.compile_shader)(v_shader);
			self.error();
			if cfg!(debug_assertions) {
				let mut value = mem::uninitialized();

				(self.get_shader)(v_shader,
					0x8B81 /*GL_COMPILE_STATUS*/,
					&mut value);
				self.error();

				if value == 0 {
					let mut value = mem::uninitialized();
					(self.get_shader)(v_shader,
						0x8B84 /*GL_INFO_LOG_LENGTH*/,
						&mut value);

					self.error();
					let mut buffer : Vec<u8> =
						vec![mem::uninitialized();
							value as usize];
					(self.info_log)(v_shader,
						value as GLsizei,
						ptr::null_mut(),
						buffer.as_mut_ptr() as *mut _);
					self.error();

					panic!("Failed to compile: {}.",
						::std::str::from_utf8(
							buffer.as_slice())
							.unwrap());
				}
			}

			let f_shader = (self.create_shader)(0x8B30/*fragment*/);
			self.error();
			(self.shader_source)(f_shader, 1,
				[fragment.as_ptr() as *const _].as_ptr(), ptr::null());
			self.error();
			(self.compile_shader)(f_shader);
			self.error();
			//		if cfg!(debug) { // TODO
			// unsafe {
				let mut value = mem::uninitialized();

				(self.get_shader)(f_shader,
					0x8B81 /*GL_COMPILE_STATUS*/,
					&mut value);
				self.error();

				if value == 0 {
					let mut value = mem::uninitialized();
					(self.get_shader)(f_shader,
						0x8B84 /*GL_INFO_LOG_LENGTH*/,
						&mut value);

					self.error();
					let mut buffer : Vec<u8> =
						vec![mem::uninitialized();
							value as usize];
					(self.info_log)(f_shader,
						value as GLsizei,
						ptr::null_mut(),
						buffer.as_mut_ptr() as *mut _);
				println!("E6");
					self.error();

					panic!("Failed to compile: {}.",
						::std::str::from_utf8(
							buffer.as_slice())
							.unwrap());
				}
			// }
//		}

			program = (self.create_program)();
			self.error();

			(self.attach_shader)(program, v_shader);
			self.error();
			(self.attach_shader)(program, f_shader);
			self.error();

			(self.link_program)(program);
			self.error();
		}

		program
	}

	/// Get uniform from a shader.
	pub fn uniform(&self, shader: u32, name: &[u8]) -> i32 {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');

		unsafe {
			let r = (self.uniform)(shader, name.as_ptr() as *const _);
			if r == -1 {
				panic!("Error No Uniform: {:?}",
					::std::str::from_utf8(name).unwrap());
			}
			self.error();
			r
		}
	}

	/// Get the attribute
	pub fn attribute(&self, shader: u32, name: &[u8]) -> Attribute {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');

		let attrib = unsafe {
			let a = (self.attribute)(shader, name.as_ptr() as *const _);
			if a == -1 {
				panic!("Error No Attribute: {:?}",
					::std::str::from_utf8(name).unwrap());
			}
			self.error();

			(self.enable_vattrib)(a as u32);
			self.error();

			a
		};

		Attribute(attrib)
	}

	/// Create some new buffers
	pub fn new_buffers(&self, n: usize) -> Vec<u32> {
		unsafe {
			let mut buffers = vec![mem::uninitialized(); n];

			(self.gen_buffers)(n as i32, buffers.as_mut_ptr());
			self.error();

			buffers
		}
	}

	/// Bind a buffer from `new_buffers()`
	pub fn bind_buffer(&self, buffer: u32) {
		unsafe {
			(self.bind_buffer)(GL_ARRAY_BUFFER, buffer);
			self.error();
		}
	}

	/// Set the bound buffer's data
	pub fn set_buffer<T>(&self, data: &[T]) {
		unsafe {
			(self.buffer_data)(GL_ARRAY_BUFFER,
				(data.len() * mem::size_of::<T>()) as isize,
				data.as_ptr() as *const _,
				GL_DYNAMIC_DRAW);
			self.error();
		}
	}

	// TODO: this actually unsafe because uniforms can only be accessed when
	// their program is in use.
	/// Use a program.
	pub fn use_program(&self, shader: u32) {
		unsafe {
			(self.use_program)(shader);
			self.error();
		}
	}

	/// Set a mat4 uniform
	pub fn set_mat4(&self, uniform: i32, mat4: &[f32; 16]) -> () {
		unsafe {
			(self.uniform_mat4)(uniform, 1, 0 /*bool: transpose*/,
				mat4.as_ptr());
			self.error();
		}
	}

	/// Set an int uniform 
	pub fn set_int1(&self, uniform: i32, int1: i32) -> () {
		unsafe {
			(self.uniform_int1)(uniform, int1);
			self.error();
		}
	}

	/// Set a float uniform
	pub fn set_vec1(&self, uniform: i32, vec1: f32) -> () {
		unsafe {
			(self.uniform_vec1)(uniform, vec1);
			self.error();
		}
	}

	/// Set a vec2 uniform
	pub fn set_vec2(&self, uniform: i32, vec: &[f32; 2]) -> () {
		unsafe {
			(self.uniform_vec2)(uniform, vec[0], vec[1]);
			self.error();
		}
	}

	/// Set a vec3 uniform
	pub fn set_vec3(&self, uniform: i32, vec: &[f32; 3]) -> () {
		unsafe {
			(self.uniform_vec3)(uniform, vec[0], vec[1], vec[2]);
			self.error();
		}
	}

	/// Set a vec4 uniform
	pub fn set_vec4(&self, uniform: i32, vec: &[f32; 4]) -> () {
		unsafe {
			(self.uniform_vec4)(uniform, vec[0], vec[1], vec[2],
				vec[3]);
			self.error();
		}
	}

	/// Draw the elements.
	pub fn draw_arrays(&self, start_index: u32, n_indices: u32) {
		unsafe {
			// draw
			(self.draw_arrays)(0x0006 /*GL_TRIANGLE_FAN*/,
				start_index as GLint, n_indices as GLsizei);
			self.error();
		}
	}

	/// Create a new texture.
	pub fn new_texture(&self) -> Texture {
		Texture(unsafe {
			let mut a = mem::uninitialized();

			(self.gen_textures)(1, &mut a);
			self.error();

			self.use_texture(&Texture(a));

			(self.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
				GL_LINEAR);
			self.error();
			(self.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
				GL_LINEAR_MIPMAP_LINEAR);
			self.error();

			a
		})
	}

	/// Set the bound texture's pixels
	pub fn set_texture(&self, w: u32, h: u32, px: &[u32]) -> () {
		unsafe {
			(self.tex_image)(GL_TEXTURE_2D, 0, GL_RGBA as i32,
				w as i32, h as i32, 0, GL_RGBA,
				GL_UNSIGNED_BYTE, px.as_ptr() as *const _);
			self.error();
			(self.gen_mipmap)(GL_TEXTURE_2D);
			self.error();
		}
	}

	/// Update the pixels of an already bound & set texture.
	pub fn texture_update(&self, w: u32, h: u32, px: &[u32]) -> () {
		unsafe {
			(self.tex_subimage)(GL_TEXTURE_2D, 0, 0, 0,
				w as i32, h as i32, GL_RGBA,
				GL_UNSIGNED_BYTE, px.as_ptr() as *const _);
			self.error();
		}
	}

	/// Use a texture.
	pub fn use_texture(&self, texture: &Texture) {
		unsafe {
			(self.bind_texture)(GL_TEXTURE_2D, texture.0);
			self.error();
		}
	}

	/// Set vertex attribute to current buffer.
	pub fn vertex_attrib(&self, attrib: &Attribute) {
		unsafe {
			(self.vertex_attrib)(attrib.0 as GLuint, 4, GL_FLOAT, 0,
				0, ptr::null());
			self.error();
		}
	}

	/// Update the viewport.
	pub fn viewport(&self, w: u32, h: u32) {
		unsafe {
			(self.viewport)(0, 0, w as GLsizei, h as GLsizei);
			self.error();
		}
	}

	#[cfg(not(debug_assertions))]
	unsafe fn error(&self) { /* Do nothing in release mode for speed. */ }

	#[cfg(debug_assertions)]
	unsafe fn error(&self) {
		match (self.get_error)() {
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
}
