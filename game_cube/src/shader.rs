use std::{ffi::{CStr, CString}, fs};

use glad_gl::gl;

pub struct Shader {
    id: gl::GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let mut shader = Shader { id: 0 };

        shader.id = shader
            .load_shaders(vertex_path, fragment_path)
            .expect("Shaders can not be loaded");
        shader
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_attrib_location(&self, attr: &str) -> gl::GLuint {
        let name: CString = CString::new(attr).unwrap();
        unsafe {
            gl::GetAttribLocation(self.id, name.as_ptr())
                .try_into()
                .unwrap()
        }
    }

    pub fn get_uniform_location(&self, attr: &str) -> gl::GLuint {
        let name: CString = CString::new(attr).unwrap();
        unsafe {
            gl::GetUniformLocation(self.id, name.as_ptr())
                .try_into()
                .unwrap()
        }
    }

    fn load_shader(
        &self,
        path: &str,
        shader_type: gl::GLenum,
    ) -> Result<gl::GLuint, Box<dyn std::error::Error>> {
        let vertex_shader_text = CString::new(fs::read(path)?)?;

        unsafe {
            let vertex_shader = gl::CreateShader(shader_type);
            let vertex_shader_array: [*const gl::GLchar; 1] = [vertex_shader_text.as_ptr() as _];
            let vertex_shader_lengths: [gl::GLint; 1] =
                [vertex_shader_text.as_bytes().len().try_into().unwrap()];
            gl::ShaderSource(
                vertex_shader,
                1,
                vertex_shader_array.as_ptr(),
                vertex_shader_lengths.as_ptr(),
            );
            gl::CompileShader(vertex_shader);

            let success = 2;
            let raw = &success as *const gl::GLint;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, raw.cast_mut());

            if success == 0 {
                let b = [0 as gl::GLchar; 256];
                gl::GetShaderInfoLog(
                    vertex_shader,
                    255,
                    std::ptr::null::<i32>() as *mut i32,
                    (&b as *const gl::GLchar).cast_mut(),
                );

                let err_text = CStr::from_ptr(b.as_ptr());
                let err_text = err_text.to_owned();
                println!(
                    "Shader compilation error, {:?}, {:?}",
                    shader_type, err_text
                );

                gl::DeleteShader(vertex_shader);
                return Err(Box::from(err_text.to_str()?));
            }

            Ok(vertex_shader)
        }
    }

    fn load_shaders(
        &self,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<gl::GLuint, Box<dyn std::error::Error>> {
        let vertex_shader = self.load_shader(vertex_path, gl::VERTEX_SHADER)?;
        let fragment_shader = self.load_shader(fragment_path, gl::FRAGMENT_SHADER)?;

        unsafe {
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            let success = 2;
            let raw = &success as *const gl::GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, raw.cast_mut());

            if success == 0 {
                let b = [0 as gl::GLchar; 256];
                gl::GetShaderInfoLog(
                    program_id,
                    255,
                    std::ptr::null::<i32>() as *mut i32,
                    (&b as *const gl::GLchar).cast_mut(),
                );

                let err_text = CStr::from_ptr(b.as_ptr());
                let err_text = err_text.to_owned();
                println!("Program link error, {:?}", err_text);

                return Err(Box::from(err_text.to_str()?));
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(program_id)
        }
    }
}