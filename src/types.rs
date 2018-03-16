// types.rs -- Aldaron's System Interface / OpenGL
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use libc;

// GL Types
#[allow(unused)] pub type GLuint = libc::c_uint;
#[allow(unused)] pub type GLint = libc::c_int;
#[allow(unused)] pub type GLenum = libc::c_uint;
#[allow(unused)] pub type GLboolean = libc::c_uchar;
#[allow(unused)] pub type GLsizei = libc::c_int;
#[allow(unused)] pub type GLchar = libc::c_char;
#[allow(unused)] pub type GLbitfield = libc::c_uint;
#[allow(unused)] pub type GLsizeiptr = isize;
#[allow(unused)] pub type GLfloat = libc::c_float;
#[allow(unused)] pub type GLubyte = libc::c_uchar;

// X11 & Android
#[allow(unused)] pub type EGLSurface = *mut libc::c_void;
#[allow(unused)] pub type EGLNativeWindowType = *mut libc::c_void;
#[allow(unused)] pub type EGLNativeDisplayType = *mut libc::c_void;
#[allow(unused)] pub type EGLDisplay = *mut libc::c_void;
#[allow(unused)] pub type EGLint = libc::int32_t;
#[allow(unused)] pub type EGLBoolean = libc::c_uint;
#[allow(unused)] pub type EGLConfig = *mut libc::c_void;
#[allow(unused)] pub type EGLContext = *mut libc::c_void;
#[allow(unused)] pub type EGLenum = libc::c_uint;

#[allow(unused)] pub const GL_FLOAT: u32 = 0x1406;
#[allow(unused)] pub const GL_TEXTURE_2D: u32 = 0x0DE1;
#[allow(unused)] pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
#[allow(unused)] pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
#[allow(unused)] pub const GL_NEAREST: i32 = 0x2600;
#[allow(unused)] pub const GL_RGBA: u32 = 0x1908;
#[allow(unused)] pub const GL_UNSIGNED_BYTE: u32 = 0x1401;

#[allow(unused)] pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
#[allow(unused)] pub const GL_ARRAY_BUFFER: u32 = 0x8892;
#[allow(unused)] pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;

#[allow(unused)] pub const EGL_BUFFER_SIZE: i32 = 0x3020;
#[allow(unused)] pub const EGL_ALPHA_SIZE: i32 = 0x3021;
#[allow(unused)] pub const EGL_BLUE_SIZE: i32 = 0x3022;
#[allow(unused)] pub const EGL_GREEN_SIZE: i32 = 0x3023;
#[allow(unused)] pub const EGL_RED_SIZE: i32 = 0x3024;
#[allow(unused)] pub const EGL_DEPTH_SIZE: i32 = 0x3025;
#[allow(unused)] pub const EGL_STENCIL_SIZE: i32 = 0x3026;
#[allow(unused)] pub const EGL_CONFIG_CAVEAT: i32 = 0x3027;
#[allow(unused)] pub const EGL_CONFIG_ID: i32 = 0x3028;	
#[allow(unused)] pub const EGL_LEVEL: i32 = 0x3029;
#[allow(unused)] pub const EGL_MAX_PBUFFER_HEIGHT: i32 = 0x302A;
#[allow(unused)] pub const EGL_MAX_PBUFFER_PIXELS: i32 = 0x302B;
#[allow(unused)] pub const EGL_MAX_PBUFFER_WIDTH: i32 = 0x302C;
#[allow(unused)] pub const EGL_NATIVE_RENDERABLE: i32 = 0x302D;
#[allow(unused)] pub const EGL_NATIVE_VISUAL_ID: i32 = 0x302E;
#[allow(unused)] pub const EGL_NATIVE_VISUAL_TYPE: i32 = 0x302F;
#[allow(unused)] pub const EGL_SAMPLES: i32 = 0x3031;
#[allow(unused)] pub const EGL_SAMPLE_BUFFERS: i32 = 0x3032;
#[allow(unused)] pub const EGL_SURFACE_TYPE: i32 = 0x3033;
#[allow(unused)] pub const EGL_TRANSPARENT_TYPE: i32 = 0x3034;
#[allow(unused)] pub const EGL_TRANSPARENT_BLUE_VALUE: i32 = 0x3035;
#[allow(unused)] pub const EGL_TRANSPARENT_GREEN_VALUE: i32 = 0x3036;
#[allow(unused)] pub const EGL_TRANSPARENT_RED_VALUE: i32 = 0x3037;
#[allow(unused)] pub const EGL_NONE: i32 = 0x3038;
#[allow(unused)] pub const EGL_BIND_TO_TEXTURE_RGB: i32 = 0x3039;
#[allow(unused)] pub const EGL_BIND_TO_TEXTURE_RGBA: i32 = 0x303A;
#[allow(unused)] pub const EGL_MIN_SWAP_INTERVAL: i32 = 0x303B;
#[allow(unused)] pub const EGL_MAX_SWAP_INTERVAL: i32 = 0x303C;

#[allow(unused)] pub const EGL_DONT_CARE: i32 = -1;
#[allow(unused)] pub const EGL_SLOW_CONFIG: i32 = 0x3050;
#[allow(unused)] pub const EGL_NON_CONFORMANT_CONFIG: i32 = 0x3051;
#[allow(unused)] pub const EGL_TRANSPARENT_RGB: i32 = 0x3052;
#[allow(unused)] pub const EGL_NO_TEXTURE: i32 = 0x305C;
#[allow(unused)] pub const EGL_TEXTURE_RGB: i32 = 0x305D;
#[allow(unused)] pub const EGL_TEXTURE_RGBA: i32 = 0x305E;
#[allow(unused)] pub const EGL_TEXTURE_2D: i32 = 0x305F;

#[allow(unused)] pub const EGL_PBUFFER_BIT: i32 = 0x01;
#[allow(unused)] pub const EGL_PIXMAP_BIT: i32 = 0x02;
#[allow(unused)] pub const EGL_WINDOW_BIT: i32 = 0x04;

#[allow(unused)] pub const EGL_VENDOR: i32 = 0x3053;
#[allow(unused)] pub const EGL_VERSION: i32 = 0x3054;
#[allow(unused)] pub const EGL_EXTENSIONS: i32 = 0x3055;

#[allow(unused)] pub const EGL_HEIGHT: i32 = 0x3056;
#[allow(unused)] pub const EGL_WIDTH: i32 = 0x3057;
#[allow(unused)] pub const EGL_LARGEST_PBUFFER: i32 = 0x3058;
#[allow(unused)] pub const EGL_TEXTURE_FORMAT: i32 = 0x3080;
#[allow(unused)] pub const EGL_TEXTURE_TARGET: i32 = 0x3081;
#[allow(unused)] pub const EGL_MIPMAP_TEXTURE: i32 = 0x3082;
#[allow(unused)] pub const EGL_MIPMAP_LEVEL: i32 = 0x3083;

#[allow(unused)] pub const EGL_BACK_BUFFER: i32 = 0x3084;

#[allow(unused)] pub const EGL_DRAW: i32 = 0x3059;
#[allow(unused)] pub const EGL_READ: i32 = 0x305A;

#[allow(unused)] pub const EGL_CORE_NATIVE_ENGINE: i32 = 0x305B;

#[allow(unused)] pub const EGL_RENDERABLE_TYPE: i32 = 0x3040;
#[allow(unused)] pub const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
#[allow(unused)] pub const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;

#[allow(unused)] pub const EGL_OPENGL_ES_API: u32 = 0x30A0;
