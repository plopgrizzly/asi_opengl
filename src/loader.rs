// loader.rs -- Aldaron's System Interface / OpenGL
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use libc;
use libc::c_void;
use std::{ mem, ptr };
use types::*;

#[cfg(target_os = "windows")]
extern "system" {
	fn LoadLibraryW(a: *const u16) -> *mut c_void;
	fn GetProcAddress(b: *mut c_void, c: *const u8) -> *mut c_void;
//	fn FreeLibrary(a: *mut c_void) -> i32; // TODO: Clean up resources on exit.
	fn SwapBuffers(a: *mut c_void) -> i32;
	fn GetDC(a: *mut c_void) -> *mut c_void;
	fn ChoosePixelFormat(a: *mut c_void, b: *const PixelFormatDescriptor)
		-> i32;
	fn SetPixelFormat(a: *mut c_void, b: i32,
		c: *const PixelFormatDescriptor) -> i32;
}

#[cfg(not(target_os = "windows"))]
#[link = "dl"]
extern "C" {
	fn dlopen(filename: *const i8, flag: i32) -> *mut c_void;
	fn dlsym(handle: *mut c_void, symbol: *const u8) -> *mut c_void;
}

#[cfg(target_os = "windows")]
unsafe fn load_lib() -> (*mut c_void, *mut c_void) {
	let vulkan = "opengl32.dll\0";
	let vulkan16 : Vec<u16> = vulkan.encode_utf16().collect();

	let lib = LoadLibraryW(vulkan16.as_ptr());
	let loader = GetProcAddress(lib, b"wglGetProcAddress\0".as_ptr());

	(loader, lib)
}

#[cfg(not(target_os = "windows"))]
unsafe fn load_lib() -> (*mut c_void, *mut c_void) {
	let egl = b"libEGL.so.1\0";
	let opengl = b"libGLESv2.so.2\0";

	let egl = dlopen(egl.as_ptr() as *const _ as *const i8, 1);

	// Prefer OpenGLES on Linux
	let opengl = dlopen(opengl.as_ptr() as *const _ as *const i8, 1);

	// OpenGL on Linux
	let opengl = if opengl.is_null() {
		let gl = b"libGL.so.1\0";

		dlopen(gl.as_ptr() as *const _ as *const i8, 1)
	} else { opengl };

	// TODO: test for libgl.so.1 is needed/works for MacOS / iOS
	// OpenGL on MacOS
	let opengl = if opengl.is_null() {
		let gl = b"libgl.so.1\0";

		dlopen(gl.as_ptr() as *const _ as *const i8, 1)
	} else { opengl };

	(egl, opengl)
}

#[cfg(target_os = "windows")]
unsafe fn dl_sym<T>(lib: *mut c_void, name: &[u8]) -> T {
	let fn_ptr = GetProcAddress(lib, &name[0]);

	mem::transmute_copy::<*mut c_void, T>(&fn_ptr)
}

#[cfg(not(target_os = "windows"))]
unsafe fn dl_sym<T>(lib: *mut c_void, name: &[u8]) -> T {
	let fn_ptr = dlsym(lib, name.as_ptr());

	mem::transmute_copy::<*mut c_void, T>(&fn_ptr)
}

#[cfg(target_os = "windows")]
pub struct Display {
	dc: Option<ptr::NonNull<libc::c_void>>, // A Windows Device Context
}

#[cfg(target_os = "windows")]
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

#[cfg(not(target_os = "windows"))]
pub struct Display {
	display: *mut libc::c_void,
	surface: Option<ptr::NonNull<libc::c_void>>,
	config: *mut libc::c_void,
	context: *mut libc::c_void,
	swap: unsafe extern "C" fn(EGLDisplay, EGLSurface) -> EGLBoolean,
}

#[cfg(not(target_os = "windows"))]
impl Display {
	// Swap surface with screen buffer.
	pub fn swap(&self) {
		if unsafe {
			(self.swap)(self.display,self.surface.unwrap().as_ptr())
		} == 0 {
			panic!("Swapping Failed");
		}
	}
}

pub struct Lib {
	// EGL .so
	#[cfg(not(target_os = "windows"))]
	handle: *mut c_void,
	// Windows OpenGL loader function
	#[cfg(target_os = "windows")]
	loader: unsafe extern "system" fn(*const u8) -> *mut c_void,
	// OpenGLES Version 2 .so / .dll
	opengl: *mut c_void,
}

impl Lib {
	#[cfg(target_os = "windows")]
	/// Load the OpenGL libary.  `None` if can't find it.
	pub fn new() -> Option<Self> {
		let (loader, opengl) = unsafe { load_lib() };

		if loader.is_null() || opengl.is_null() {
			None
		} else {
			Some(Lib { loader: unsafe { mem::transmute_copy(&loader) }, opengl })
		}
	}

	#[cfg(not(target_os = "windows"))]
	/// Load the OpenGL libary.  `None` if can't find it.
	pub fn new() -> Option<Self> {
		let (handle, opengl) = unsafe { load_lib() };

		if handle.is_null() || opengl.is_null() {
			None
		} else {
			Some(Lib { handle, opengl })
		}
	}

	/// Initialize the opengl (connect to the display)
	#[cfg(target_os = "windows")]
	pub fn init(&self) -> (Display, i32) {
		(Display {
			dc: None,
		}, 0)
	}
	
	/// Initialize the opengl (connect to the display)
	#[cfg(not(target_os = "windows"))]
	pub fn init(&self) -> (Display, i32) {
		// EGL
		let get_display: unsafe extern "C" fn(EGLNativeDisplayType)
			-> EGLDisplay = unsafe {
				dl_sym(self.handle, b"eglGetDisplay\0")
			};
		let initialize: unsafe extern "C" fn(EGLDisplay, *mut EGLint,
			*mut EGLint) -> EGLBoolean = unsafe {
				dl_sym(self.handle, b"eglInitialize\0")
			};
		let choose_config: unsafe extern "C" fn(EGLDisplay, 
			*const EGLint, *mut EGLConfig, EGLint, *mut EGLint)
				-> EGLBoolean
			= unsafe {
				dl_sym(self.handle, b"eglChooseConfig\0")
			};
		let create_context: unsafe extern "C" fn(EGLDisplay,
			EGLConfig, EGLContext, *const EGLint) -> EGLContext
			= unsafe {
				dl_sym(self.handle, b"eglCreateContext\0")
			};
		let get_ca: unsafe extern "C" fn(EGLDisplay, EGLConfig, EGLint,
				*mut EGLint) -> EGLBoolean
			= unsafe {
				dl_sym(self.handle, b"eglGetConfigAttrib\0")
			};
		let bind_api: unsafe extern "system" fn(EGLenum) -> EGLBoolean
			= unsafe {
				dl_sym(self.handle, b"eglBindAPI\0")
			};
		let swap = unsafe {
			dl_sym(self.handle, b"eglSwapBuffers\0")
		};

		let display = unsafe { (get_display)(ptr::null_mut()) };
		if display.is_null() {
			panic!("EGL: Couldn't load display.");
		}

		if unsafe {
			(initialize)(display,ptr::null_mut(),ptr::null_mut())
		} == 0 {
			panic!("Couldn't initialize EGL");
		}

		// Config
		let mut config = ptr::null_mut();
		let mut nconfigs = unsafe { mem::uninitialized() };

		if unsafe {
			(choose_config)(display, [
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

		if unsafe { (bind_api)(EGL_OPENGL_ES_API) } == 0 {
			panic!("Couldn't bind OpenGLES");
		}

		// Create an EGL rendering context.
		let context = unsafe {
			(create_context)(display, config, ptr::null_mut(),
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
			get_ca(display, config, EGL_NATIVE_VISUAL_ID,
				&mut visual_id)
		} == 0 {
			panic!("couldn't get visual id");
		}

		(Display {
			display,
			surface,
			swap,
			config,
			context,
		}, visual_id)
	}
	
	#[cfg(target_os = "windows")]
	pub fn init2(&self, display: &mut Display, window: *mut libc::c_void) {
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
			-> *mut c_void
			= unsafe {
				dl_sym(self.opengl, b"wglCreateContext\0")
			};
		let make_current: unsafe extern "system" fn(*mut c_void,
			*mut c_void) -> i32
			= unsafe {
				dl_sym(self.opengl, b"wglMakeCurrent\0")
			};
		
		unsafe {
			let context = create_context(dc);
			make_current(dc, context);
		}
	}

	/// Initialize the opengl (connect to the display) STEP 2
	#[cfg(not(target_os = "windows"))]
	pub fn init2(&self, display: &mut Display, window: EGLNativeWindowType){
		let create_window_surface: unsafe extern "C" fn(EGLDisplay, 
			EGLConfig, EGLNativeWindowType, *const EGLint)
				-> EGLSurface
			= unsafe {
				dl_sym(self.handle, b"eglCreateWindowSurface\0")
			};
		let make_current: unsafe extern "C" fn(EGLDisplay, EGLSurface,
			EGLSurface, EGLContext) -> EGLBoolean = unsafe {
				dl_sym(self.handle, b"eglMakeCurrent\0")
			};

		// Create surface
		let surface = unsafe {
			(create_window_surface)(display.display,
				display.config, window, ptr::null())
		};

		if surface.is_null() {
			panic!("Couldn't create EGL surface.");
		}

		// Connect context to surface
		if unsafe {
			(make_current)(display.display, surface, surface,
				display.context)
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

	#[cfg(not(target_os = "windows"))]
	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let fn_ptr: *const c_void = unsafe {
			dl_sym(self.opengl, name)
		};

		self.load_check(name, fn_ptr);

		unsafe { mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
	}

	#[cfg(target_os = "windows")]
	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let mut fn_ptr: *const c_void = unsafe {
			(self.loader)(name.as_ptr())
		};
		
		if fn_ptr.is_null() {
			fn_ptr = unsafe { dl_sym(self.opengl, name) };
		}

		self.load_check(name, fn_ptr);

		unsafe { mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
	}
}
