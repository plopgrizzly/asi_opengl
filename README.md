# Aldaron's System Interface / OpenGL

Rust safe bindings for OpenGL / OpenGLES

[Cargo](https://crates.io/crates/asi_opengl) /
[Documentation](https://docs.rs/asi_opengl)

## Features
**asi_opengl**'s current features:
* Somewhat safe OpenGL/ES bindings

**asi_opengl**'s planned features:
* Totally safe OpenGL/ES bindings
* Speed enchancements: Like checking whether or not a uniform in a shader is already set to the value we're trying to set (stored in shader struct).

## Support
**asi_opengl**'s current support:
* EGL/OpenGLES with XCB
* WGL/OpenGL on Windows

**asi_opengl**'s planned support:
* EGL/OpenGLES with Wayland
* EGL/OpenGLES on Android
* MacOS

# Contributing
If you'd like to help implement functions for unsupported platforms, fix bugs,
improve the API or improve the Documentation, then contact me at
jeron.lau@plopgrizzly.com. I'll appreciate any help.
