use sdl2::video::{GLContext, Window};
use crate::renderer::commands::{Command, Commands};
use crate::renderer::renderer;

use gl::types::GLuint;
use sdl2::VideoSubsystem;

pub struct OpenGL {
    gl_ctx: GLContext,
    settings: renderer::Settings,

    vertex_array: GLuint,
    vertex_buffer: GLuint,
    index_buffer: GLuint,
}

impl renderer::Backend for OpenGL {
    fn execute_commands(&mut self, commands: Commands) {
        self.prepare_draw();
        self.execute(commands);
        self.finalize_draw();
    }
}

impl OpenGL {
    pub fn new(settings: renderer::Settings, window: &Window, video_subsystem: &VideoSubsystem) -> Self {
        let gl_ctx = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        Self {
            gl_ctx,
            settings,

            vertex_array: 0,
            vertex_buffer: 0,
            index_buffer: 0,
        }
    }

    fn prepare_draw(&mut self) {}

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
                Command::Quad(q) => {}
                Command::Setup(s) => {}
            }
        }
    }
}