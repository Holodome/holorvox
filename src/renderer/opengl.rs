use std::mem::size_of;
use sdl2::video::{GLContext, Window};
use crate::renderer::commands::{Command, Commands};
use crate::renderer::renderer;

use gl::types::{GLuint, GLenum, GLint};
use sdl2::VideoSubsystem;
use crate::renderer::misc::Vertex;
use crate::renderer::renderer::Settings;

fn create_opengl_shader(source: &str, defines: &str) -> GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

        const GLOBAL_SHADER_CODE: &str = "#version 330";

        let vertex_source = vec![
            GLOBAL_SHADER_CODE, "#define VERTEX_SHADER", defines, source,
        ];
        let fragment_source = vec![
            GLOBAL_SHADER_CODE, defines, source,
        ];
        gl::ShaderSource(vertex_shader, vertex_source.len().try_into().unwrap(),
                         vertex_source.as_ptr() as *const *const i8, 0 as *const _);
        gl::ShaderSource(fragment_shader, fragment_source.len().try_into().unwrap(),
                         fragment_source.as_ptr() as *const *const i8, 0 as *const _);

        let mut vertex_compiled: GLint;
        let mut fragment_compiled: GLint;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut vertex_compiled);
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut fragment_compiled);

        if vertex_compiled == 0 || fragment_compiled == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(4096);
            let mut log_len = 0_i32;
            if vertex_compiled == 0 {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    4096,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            } else if fragment_compiled == 0 {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    4096,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }
        }

        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex_shader);
        gl::AttachShader(id, fragment_shader);
        gl::LinkProgram(id);

        let mut link_success: GLint;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut link_success);
        if link_success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(4096);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
                fragment_shader,
                4096,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Shader link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        id
    }
}

struct OpenGLQuadShader {
    id: GLuint,
    view_location: GLuint,
    proj_location: GLuint,
}

impl OpenGLQuadShader {
    fn new() -> Self {
        const SHADER: &str = r#"
        #ifdef VERTEX_SHADER
        layout(location = 0) in vec4 position;
        layout(location = 1) in vec2 uv;
        layout(location = 2) in vec4 color;

        out vec4 frag_color;
        out vec4 frag_uv;

        uniform mat4 view_matrix = mat4(1);
        uniform mat4 proj_matrix = mat4(1);

        void main() {
            vec4 world_space = position;
            vec4 cam_space = view_matrix * world_space;
            vec4 clip_space = proj_matrix * cam_space;
            gl_Position = clip_space;

            frag_color = color;
            frag_uv = uv;
        }
        #else
        in vec4 frag_color;
        in vec4 frag_uv;

        out vec4 out_color;

        void main() {
            out_color = frag_color;
        }
        #endif
        "#;
        let id = create_opengl_shader(SHADER, "");
    }

    fn bind(&self) {}
}

pub struct OpenGL {
    gl_ctx: GLContext,
    settings: renderer::Settings,

    vertex_array: GLuint,
    vertex_buffer: GLuint,
    index_buffer: GLuint,
}

impl renderer::Backend for OpenGL {
    fn execute_commands(&mut self, commands: Commands) {
        self.prepare_draw(&commands);
        self.execute(commands);
        self.finalize_draw();
    }
}

struct OpenGLInitBuffers {
    vertex_array: GLuint,
    vertex_buffer: GLuint,
    index_buffer: GLuint,
}

impl OpenGL {
    pub fn new(settings: Settings, window: &Window, video_subsystem: &VideoSubsystem) -> Self {
        let gl_ctx = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let buffers = Self::initialize_buffers(&settings);

        Self {
            gl_ctx,
            settings,

            vertex_array: buffers.vertex_array,
            vertex_buffer: buffers.vertex_buffer,
            index_buffer: buffers.index_buffer,
        }
    }

    fn initialize_buffers(settings: &Settings) -> OpenGLInitBuffers {
        unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            assert_ne!(vertex_array, 0);
            gl::BindVertexArray(vertex_array);

            let mut vertex_buffer = 0;
            gl::GenBuffers(1, &mut vertex_buffer);
            assert_ne!(vertex_array, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (settings.max_vertex_count * std::mem::size_of::<Vertex>()).try_into().unwrap(), std::ptr::null(), gl::STREAM_DRAW);

            let mut index_buffer = 0;
            gl::GenBuffers(1, &mut index_buffer);
            assert_ne!(index_buffer, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (settings.max_index_count * std::mem::size_of::<usize>()).try_into().unwrap(), std::ptr::null(), gl::STREAM_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of::<Vertex>()
                .try_into().unwrap(), 0 as *const _);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size_of::<Vertex>()
                .try_into().unwrap(), (3 * size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, size_of::<Vertex>()
                .try_into().unwrap(), (5 * size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(2);

            gl::BindVertexArray(0);

            OpenGLInitBuffers {
                vertex_array,
                vertex_buffer,
                index_buffer,
            }
        }
    }

    fn prepare_draw(&mut self, commands: &Commands) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
            gl::Viewport(0, 0, self.settings.viewport_size.x as i32, self.settings.viewport_size.y as i32);
            gl::DepthMask(gl::TRUE);
            gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
            gl::DepthFunc(gl::LEQUAL);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CCW);
            gl::ProvokingVertex(gl::FIRST_VERTEX_CONVENTION);

            gl::BindVertexArray(self.vertex_array);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (commands.vertices.len() * size_of::<Vertex>())
                .try_into().unwrap(),
                              commands.vertices.as_ptr() as *const _);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (commands.indices.len() * size_of::<usize>())
                .try_into().unwrap(),
                              commands.indices.as_ptr() as *const _);
            gl::BindVertexArray(0);
        }
    }

    fn finalize_draw(&mut self) {}

    fn execute(&mut self, commands: Commands) {
        for command in commands.commands {
            match command {
                Command::Clear(c) => {
                    unsafe {
                        gl::ClearColor(c.x, c.y, c.z, c.w);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }
                }
                Command::Quad(q) => {
                    unsafe {
                        gl::BindVertexArray(self.vertex_array);
                        gl::DrawElementsBaseVertex(gl::TRIANGLES, (6 * q.quad_count).try_into().unwrap(),
                                                   Self::get_gl_index_type(),
                                                   (size_of::<usize>() * q.index_buffer_offset) as
                                                       *const _, q.vertex_buffer_offset
                                                       .try_into().unwrap());
                        gl::BindVertexArray(0);
                    }
                }
                Command::Setup(s) => {}
            }
        }
    }

    fn get_gl_index_type() -> GLenum {
        match size_of::<usize>() {
            4 => gl::UNSIGNED_INT,
            2 => gl::UNSIGNED_SHORT,
            1 => gl::UNSIGNED_BYTE,
            _ => panic!("unexpected usize size")
        }
    }
}