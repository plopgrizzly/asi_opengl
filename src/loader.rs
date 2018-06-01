// "asi_opengl" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use c_void;
use std::{ mem, ptr };
use types::*;

#[cfg(windows)]
use winapi::shared::ntdef::LPCSTR;

#[cfg(windows)]
dl_api!(WinOpenGL, "opengl32.dll",
	fn wglGetProcAddress(LPCSTR) -> *mut c_void
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
			(lib.egl.eglSwapBuffers)(self.display,
				self.surface.unwrap().as_ptr())
		} == 0 {
			panic!("Swapping Failed");
		}
	}
}

pub struct Lib {
	#[cfg(not(windows))]
	egl: UnixEGL,
	#[cfg(windows)]
	wgl: WinOpenGL,
}

impl Lib {
	#[cfg(windows)]
	/// Load the OpenGL libary.  `None` if can't find it.
	pub fn new() -> Option<Self> {
		let wgl = WinOpenGL::new();

		if wgl.is_err() {
			None
		} else {
			let wgl = wgl.unwrap(); // is Ok
			Some(Lib { wgl })
		}
	}

	#[cfg(not(windows))]
	/// Load the OpenGL libary.  `None` if can't find it.
	pub fn new() -> Option<Self> {
		let egl = UnixEGL::new();

		if egl.is_err() {
			None
		} else {
			let egl = egl.unwrap(); // isn't None
			Some(Lib { egl })
		}
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
			(self.egl.eglGetDisplay)(ptr::null_mut())
		};
		if display.is_null() {
			panic!("EGL: Couldn't load display.");
		}

		if unsafe {
			(self.egl.eglInitialize)(display, ptr::null_mut(),
				ptr::null_mut())
		} == 0 {
			panic!("Couldn't initialize EGL");
		}

		// Config
		let mut config = ptr::null_mut();
		let mut nconfigs = unsafe { mem::uninitialized() };

		if unsafe {
			(self.egl.eglChooseConfig)(display, [
				EGL_RED_SIZE, 8,
				EGL_GREEN_SIZE, 8,
				EGL_BLUE_SIZE, 8,
				EGL_DEPTH_SIZE, 24,
				EGL_SAMPLE_BUFFERS, 1,
				EGL_SAMPLES, 8,
				EGL_NONE
			].as_ptr(), &mut config, 1, &mut nconfigs)
		} == 0 {
			panic!("Couldn't choose the config");
		}

		if nconfigs == 0 {
			panic!("No configs!");
		}

		if unsafe { (self.egl.eglBindAPI)(EGL_OPENGL_ES_API) } == 0 {
			panic!("Couldn't bind OpenGLES");
		}

		// Create an EGL rendering context.
		let context = unsafe {
			(self.egl.eglCreateContext)(display, config,
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
			(self.egl.eglGetConfigAttrib)(display, config,
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
			c_color_bits: 32,
			c_red_bits: 0, c_red_shift: 0, c_green_bits: 0,
			c_green_shift: 0, c_blue_bits: 0, c_blue_shift: 0,
			c_alpha_bits: 0, c_alpha_shift: 0, c_accum_bits: 0,
			c_accum_red_bits: 0, c_accum_green_bits: 0,
			c_accum_blue_bits: 0, c_accum_alpha_bits: 0,
			c_depth_bits: 24,
			c_stencil_bits: 0, c_aux_buffers: 0,
			i_layer_type: 0 /*main-plane*/,
			b_reserved: 0, dw_layer_mask: 0, dw_visible_mask: 0,
			dw_damage_mask: 0,
		};
		
		let format = unsafe {
			ChoosePixelFormat(dc, &pixel_format)
		};
		
		unsafe {
			SetPixelFormat(dc, format, &pixel_format);
		}
		
		let create_context: unsafe extern "system" fn(*mut c_void)
			-> *mut c_void = self.load(b"wglCreateContext\0");
		let make_current: unsafe extern "system" fn(*mut c_void,
			*mut c_void) -> i32 = self.load(b"wglMakeCurrent\0");

		// TODO: wglChoosePixelFormat, for multisampling(8) on Windows.
		
		unsafe {
			let context = create_context(dc);
			make_current(dc, context);
		}
	}

	/// Initialize the opengl (connect to the display) STEP 2
	#[cfg(not(windows))]
	pub fn init2(&self, display: &mut Display, window: EGLNativeWindowType){
		// Create surface
		let surface = unsafe {
			(self.egl.eglCreateWindowSurface)(display.display,
				display.config, window, ptr::null())
		};

		if surface.is_null() {
			panic!("Couldn't create EGL surface.");
		}

		// Connect context to surface
		if unsafe {
			(self.egl.eglMakeCurrent)(display.display, surface,
				surface, display.context)
		} == 0 {
			panic!("Couldn't make current");
		}

		// Guaranteed to be `Some` because of conditional panic above.
		display.surface = ptr::NonNull::new(surface);
	}

	fn load_check(&self, name: &[u8], fn_ptr: *const c_void) {
		if fn_ptr.is_null() {
			panic!("couldn't load function \"{}\"!", ::std::str::from_utf8(name).unwrap());
		}
	}

	#[cfg(not(windows))]
	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let fn_ptr: *const c_void = unsafe {
			(self.egl.eglGetProcAddress)(name as *const _
				as *const i8)
		};

		self.load_check(name, fn_ptr);

		unsafe { mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
	}

	#[cfg(windows)]
	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let fn_ptr: *const c_void = unsafe {
			(self.wgl.wglGetProcAddress)(name as *const _ as LPCSTR)
		};

		self.load_check(name, fn_ptr);

		unsafe { mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
	}
}
