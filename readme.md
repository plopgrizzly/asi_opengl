# [Aldaron's System Interface / OpenGL](https://crates.io/crates/asi_opengl)
Rust safe bindings for OpenGL / OpenGLES

## Features
* Safe OpenGL/ES bindings (Works on both Linux (through XCB) and Windows)

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* Use `awi` as dependency to create window.
* Support Android
* Support Wayland

## Change Log
## 0.5
* `set_mat4` no longer takes a reference to the matrix.

## 0.4
* Safe API
* Use LINEAR instead of NEAREST for texturing.
* Use mipmapping to speed up drawing big textures in small areas.

## 0.3
* Uses sliced triangle fans now.

## 0.2
* Support for OpenGL on Linux actually works now.

## 0.1
* Initial release

## Developed by [Plop Grizzly](http://plopgrizzly.com)
