use anyhow::bail;
use glad_gl::gl::{self, GLchar, GLfloat, GLint, GLsizei, GLuint};
use nalgebra::{Matrix4, Vector3};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Shader {
    id: GLuint,
    uniforms: HashMap<&'static str, GLint>,
}

impl Shader {
    pub fn new(
        vertex_src: &str,
        fragment_src: &str,
        uniforms: &[(&'static str, GLint)],
    ) -> anyhow::Result<Self> {
        let id;
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex,
                1,
                &(vertex_src.as_ptr() as *const GLchar),
                &(vertex_src.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex);
            let mut is_compiled: GLint = 0;
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut is_compiled);
            if is_compiled == gl::FALSE as i32 {
                let mut max_length: GLsizei = 0;
                gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut max_length);

                let mut error: Vec<u8> = Vec::with_capacity(max_length.try_into().unwrap());
                let mut length: GLsizei = 0;
                gl::GetShaderInfoLog(
                    vertex,
                    max_length,
                    &mut length,
                    error.as_mut_ptr() as *mut i8,
                );
                error.set_len(length.try_into().unwrap());

                gl::DeleteShader(vertex);
                bail!(
                    "failed to compile vertex shader: {}",
                    String::from_utf8(error).unwrap()
                );
            }

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment,
                1,
                &(fragment_src.as_ptr() as *const GLchar) as *const *const GLchar,
                &fragment_src.len().try_into().unwrap() as *const GLint,
            );
            gl::CompileShader(fragment);
            let mut is_compiled: GLint = 0;
            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut is_compiled as *mut GLint);
            if is_compiled == gl::FALSE as i32 {
                let mut max_length: GLsizei = 0;
                gl::GetShaderiv(fragment, gl::INFO_LOG_LENGTH, &mut max_length);

                let mut error: Vec<u8> = Vec::with_capacity(max_length.try_into().unwrap());
                let mut length: GLsizei = 0;
                gl::GetShaderInfoLog(
                    fragment,
                    max_length,
                    &mut length,
                    error.as_mut_ptr() as *mut i8,
                );
                error.set_len(length.try_into().unwrap());

                gl::DeleteShader(fragment);
                gl::DeleteShader(vertex);
                bail!(
                    "failed to compile fragment shader: {}",
                    String::from_utf8(error).unwrap()
                );
            }

            id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            let mut is_linked: GLint = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut is_linked);
            if is_linked == gl::FALSE as i32 {
                let mut max_length: GLsizei = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut max_length);

                let mut error: Vec<u8> = Vec::with_capacity(max_length.try_into().unwrap());
                let mut length: GLsizei = 0;
                gl::GetProgramInfoLog(id, max_length, &mut length, error.as_mut_ptr() as *mut i8);
                error.set_len(length.try_into().unwrap());

                gl::DeleteShader(fragment);
                gl::DeleteShader(vertex);
                bail!(
                    "failed to link shader: {}",
                    String::from_utf8(error).unwrap()
                );
            }

            gl::DeleteShader(fragment);
            gl::DeleteShader(vertex);
        }

        Ok(Self {
            id,
            uniforms: HashMap::from_iter(uniforms.iter().copied()),
        })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform_3f(&self, name: &'static str, v: Vector3<GLfloat>) {
        unsafe {
            gl::Uniform3f(self.uniforms[name], v.x, v.y, v.z);
        }
    }

    // pub fn set_uniform_4f(&self, name: &'static str, v: Vector4<GLfloat>) {
    //     unsafe {
    //         gl::Uniform4f(self.uniforms[name], v.x, v.y, v.z, v.w);
    //     }
    // }

    pub fn set_uniform_mat4f(&self, name: &'static str, v: Matrix4<GLfloat>) {
        unsafe {
            gl::UniformMatrix4fv(self.uniforms[name], 1, gl::FALSE, v.as_slice().as_ptr());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
