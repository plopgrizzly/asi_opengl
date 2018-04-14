// loader.rs -- Aldaron's System Interface / OpenGL
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use libc;

use std::{ mem, ptr, ptr::NonNull };

use Void;

use types::*;

#[cfg(target_os = "windows")]
extern "system" {
	// TODO LoadLibraryA?
	fn LoadLibraryW(a: *const u16) -> *mut Void;
	fn wglGetProcAddress(c: *const u8) -> *mut Void;
	fn GetProcAddress(b: *mut Void, c: *const u8) -> *mut Void;
	fn FreeLibrary(a: *mut Void) -> i32;
}

#[cfg(not(target_os = "windows"))]
#[link = "dl"]
extern "C" {
	fn dlopen(filename: *const i8, flag: i32) -> *mut Void;
	fn dlsym(handle: *mut Void, symbol: *const u8) -> *mut Void;
}

#[cfg(target_os = "windows")]
unsafe fn load_lib() -> *mut Void {
	// TODO is necessary?
	let vulkan = "opengl32.dll\0";
	let vulkan16 : Vec<u16> = vulkan.encode_utf16().collect();

	LoadLibraryW(vulkan16.as_ptr());
}

#[cfg(not(target_os = "windows"))]
unsafe fn load_lib() -> (*mut Void, *mut Void) {
	if cfg!(any(target_os = "linux", target_os = "android")) {
		let egl = b"libEGL.so.1\0";
		let opengl = b"libGLESv2.so.2\0";

		let egl = dlopen(egl.as_ptr() as *const _ as *const i8, 1);
		let opengl = dlopen(opengl.as_ptr() as *const _ as *const i8, 1);

		(egl, opengl)
	} else { // MacOS / IOS
		let gl = b"libgl.so.1\0";

		let gl = dlopen(gl.as_ptr() as *const _ as *const i8, 1);

		(ptr::null_mut(), gl)
	}
}

#[cfg(target_os = "windows")]
unsafe fn dl_sym<T>(lib: *mut Void, name: &[u8]) -> T {
	let fn_ptr = wglGetProcAddress(&name[0]);
	let error = mem::transmute_copy::<*mut Void, isize>(&fn_ptr);

	match error {
		0x0 | 0x1 | 0x2 | 0x3 | -1 => {
			fn_ptr = GetProcAddress(lib, &name[0]);
		}
		_ => {}
	}

	mem::transmute_copy::<*mut Void, T>(&fn_ptr)
}

#[cfg(not(target_os = "windows"))]
unsafe fn dl_sym<T>(lib: *mut Void, name: &[u8]) -> T {
	let fn_ptr = dlsym(lib, name.as_ptr());

	mem::transmute_copy::<*mut Void, T>(&fn_ptr)
}

#[cfg(not(target_os = "windows"))]
pub struct Display {
	display: *mut libc::c_void,
	surface: Option<NonNull<libc::c_void>>,
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
	handle: *mut Void,
	// OpenGLES Version 2 .so / .dll
	opengl: *mut Void,
}

impl Lib {
	#[cfg(target_os = "windows")]
	/// Load the OpenGL libary.  `None` if can't find it.
	pub fn new() -> Option<Self> {
		let opengl = unsafe { load_lib() };

		if opengl.is_null() {
			None
		} else {
			Some(Lib { opengl })
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
		display.surface = NonNull::new(surface);
	}

	// Load an OpenGL 3 / OpenGLES 2 function.
	pub fn load<T>(&self, name: &[u8]) -> T {
		let fn_ptr: *const Void = unsafe { dl_sym(self.opengl, name) };

		if fn_ptr.is_null() {
			panic!("couldn't load function!");
		}

		unsafe { mem::transmute_copy::<*const Void, T>(&fn_ptr) }
	}
}
