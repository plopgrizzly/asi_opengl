# "asi_opengl" - Aldaron's System Interface - OpenGL
#
# Copyright Jeron A. Lau 2018.
# Distributed under the Boost Software License, Version 1.0.
# (See accompanying file LICENSE_1_0.txt or copy at
# https://www.boost.org/LICENSE_1_0.txt)

use c_void;
use std::{ mem, ptr };
use types::*;

#[cfg(windows)]
use winapi::shared::{
	ntdef::LPCSTR,
	minwindef::BOOL,
};

#[cfg(windows)]
dl_api!(WinOpenGL, "opengl32.dll",
	fn wglGetProcAddress(LPCSTR) -> *mut c_void,
	fn wglCreateContext(*mut c_void) -> *mut c_void,
	fn wglMakeCurrent(*mut c_void, *mut c_void) -> BOOL
);

#[cfg(not(windows))]
dl_api!(UnixEGL, "libEGL.so.1",
	fn eglGetDisplay(EGLNativeDisplayType) -> EGLDisplay,
	fn eglInitialize(EGLDisplay, *mut EGLint, *mut EGLint) -> EGLBoolean,
	fn eglChooseConfig(EGLDisplay, *const EGLint, *mut EGLConfig, EGLint,
		*mut EGLint) -> EGLBoolean,
	fn eglCreateContext(EGLDisplay, EGLConfig, EGLContext, *const EGLint)
		-> EGLContext,
	fn eglGetConfigAttrib(EGLDisplay, EGLConfig, EGLint, *mut EGLint)
		-> EGLBoolean,
	fn eglBindAPI(EGLenum) -> EGLBoolean,
	fn eglSwapBuffers(EGLDisplay, EGLSurface) -> EGLBoolean,
	fn eglGetProcAddress(*const i8) -> *mut c_void,
	fn eglCreateWindowSurface(EGLDisplay, EGLConfig, EGLNativeWindowType,
		*const EGLint) -> EGLSurface,
	fn eglMakeCurrent(EGLDisplay, EGLSurface, EGLSurface, EGLContext)
		-> EGLBoolean
);

#[cfg(windows)]
extern "system" {
	fn SwapBuffers(a: *mut c_void) -> i32;
	fn GetDC(a: *mut c_void) -> *mut c_void;
	fn ChoosePixelFormat(a: *mut c_void, b: *const PixelFormatDescriptor)
		-> i32;
	fn SetPixelFormat(a: *mut c_void, b: i32,
		c: *const PixelFormatDescriptor) -> i32;
}

#[cfg(windows)]
pub struct Display {
	dc: Option<ptr::NonNull<c_void>>, // A Windows Device Context
}

#[cfg(windows)]
impl Display {
	// Swap surface with screen buffer.
	pub fn swap(&self) {
		if unsafe {
			SwapBuffers(self.dc.unwrap().as_ptr())
		} == 0 {
			panic!("Swapping Failed");
		}
	}
}

#[cfg(not(windows))]
pub struct Display {
	display: *mut c_void,
	surface: Option<ptr::NonNull<c_void>>,
	config: *mut c_void,
	context: *mut c_void,
}

#[cfg(not(windows))]
impl Display {
	// Swap surface with screen buffer.
	pub fn swap(&self, lib: &Lib) {
		if unsafe {
			(lib.gl.eglSwapBuffers)(self.display,
				self.surface.unwrap().as_ptr())
		} == 0 {
			panic!("Swapping Failed");
		}
	}
}

pub struct Lib {
	#[cfg(not(windows))]
	gl: UnixEGL,
	#[cfg(windows)]
	gl: WinOpenGL,
}

impl Lib {
	/// Load the OpenGL libary.  `None` if can't find it.
	pub fn new() -> Result<Self, ::dl_api::Error> {
		#[cfg(windows)] type Gl = WinOpenGL;
		#[cfg(not(windows))] type Gl = UnixEGL;

		Ok(Lib { gl: Gl::new()? })
	}

	/// Initialize the opengl (connect to the display)
	#[cfg(windows)]
	pub fn init(&self) -> (Display, i32) {
		(Display {
			dc: None,
		}, 0)
	}
	
	/// Initialize the opengl (connect to the display)
	#[cfg(not(windows))]
	pub fn init(&self) -> (Display, i32) {
		let display = unsafe {
			(self.gl.eglGetDisplay)(ptr::null_mut())
		};
		if display.is_null() {
			panic!("EGL: Couldn't load display.");
		}

		if unsafe {
			(self.gl.eglInitialize)(display, ptr::null_mut(),
				ptr::null_mut())
		} == 0 {
			panic!("Couldn't initialize EGL");
		}

		// Config
		let mut config = ptr::null_mut();
		let mut nconfigs = unsafe { mem::uninitialized() };

		if unsafe {
			(self.gl.eglChooseConfig)(display, [
				EGL_RED_SIZE, 8,
				EGL_GREEN_SIZE, 8,
				EGL_BLUE_SIZE, 8,
				EGL_DEPTH_SIZE, 24,
				EGL_NONE
			].as_ptr(), &mut config, 1, &mut nconfigs)
		} == 0 {
			panic!("Couldn't choose the config");
		}

		if nconfigs == 0 {
			panic!("No configs!");
		}

		if unsafe { (self.gl.eglBindAPI)(EGL_OPENGL_ES_API) } == 0 {
			panic!("Couldn't bind OpenGLES");
		}

		// Create an EGL rendering context.
		let context = unsafe {
			(self.gl.eglCreateContext)(display, config,
				ptr::null_mut(),
				[EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE]
					.as_ptr()
			)
		};

		if context.is_null() {
			panic!("Couldn't create EGL rendering context.");
		}

		let surface = None;

		// Get visual id
		let mut visual_id = unsafe { mem::uninitialized() };
		if unsafe {
			(self.gl.eglGetConfigAttrib)(display, config,
				EGL_NATIVE_VISUAL_ID, &mut visual_id)
		} == 0 {
			panic!("couldn't get visual id");
		}

		(Display {
			display,
			surface,
			config,
			context,
		}, visual_id)
	}
	
	#[cfg(windows)]
	pub fn init2(&self, display: &mut Display, window: *mut c_void) {
		let dc = unsafe { GetDC(window) };
	
		display.dc = ptr::NonNull::new(dc);
		
		let pixel_format = PixelFormatDescriptor {
			n_size: mem::size_of::<PixelFormatDescriptor>() as u16,
			n_version: 1,
			dw_flags: 4 /*draw-to-window*/ | 32 /*support-opengl*/
				| 1 /*doublebuffer*/,
			i_pixel_type: 0 /*RGBA*/,
			c_color_bits: 24,
			c_red_bits: 0, c_red_shift: 0, c_green_bits: 0,
			c_green_shift: 0, c_blue_bits: 0, c_blue_shift: 0,
			c_alpha_bits: 0, c_alpha_shift: 0, c_accum_bits: 0,
			c_accum_red_bits: 0, c_accum_green_bits: 0,
			c_accum_blue_bits: 0, c_accum_alpha_bits: 0,
			c_depth_bits: 24,
			c_stencil_bits: 8, c_aux_buffers: 0,
			i_layer_type: 0 /*main-plane*/,
			b_reserved: 0, dw_layer_mask: 0, dw_visible_mask: 0,
			dw_damage_mask: 0,
		};
		
		let format = unsafe {
			ChoosePixelFormat(dc, &pixel_format)
		};
		
		unsafe {
			SetPixelFormat(dc, format, &pixel_format);
			
			let context = (self.gl.wglCreateContext)(dc);
			(self.gl.wglMakeCurrent)(dc, context);
		}
	}

	/// Initialize the opengl (connect to the display) STEP 2
	#[cfg(not(windows))]
	pub fn init2(&self, display: &mut Display, window: EGLNativeWindowType){
		// Create surface
		let surface = unsafe {
			(self.gl.eglCreateWindowSurface)(display.display,
				display.config, window, ptr::null())
		};

		if surface.is_null() {
			panic!("Couldn't create EGL surface.");
		}

		// Connect context to surface
		if unsafe {
			(self.gl.eglMakeCurrent)(display.display, surface,
				surface, display.context)
		} == 0 {
			panic!("Couldn't make current");
		}

		// Guaranteed to be `Some` because of conditional panic above.
		display.surface = ptr::NonNull::new(surface);
	}

	#[cfg(not(windows))]
	fn load_check(&self, name: &[u8], fn_ptr: *const c_void) {
		if fn_ptr.is_null() {
			panic!("couldn't load function \"{}\"!", ::std::str::from_utf8(name).unwrap());
		}
	}

	#[cfg(not(windows))]
	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let fn_ptr: *const c_void = unsafe {
			(self.gl.eglGetProcAddress)(name as *const _
				as *const i8)
		};

		self.load_check(name, fn_ptr);

		unsafe { mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
	}

	#[cfg(windows)]
	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let fn_ptr: *const c_void = unsafe {
			(self.gl.wglGetProcAddress)(name as *const _ as LPCSTR)
		};
		
		if fn_ptr.is_null() {
			if let Ok(n) = unsafe {
				self.gl.__lib.symbol_cstr(
					::std::ffi::CStr::from_bytes_with_nul(
						name
					).unwrap()
				)
			} {
				return n;
			} else {
				panic!("couldn't load function \"{}\"!",
					::std::str::from_utf8(name).unwrap());
			};
		}

		unsafe { mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
	}
}
