use sdl2::video::Window;
use sdl2::VideoSubsystem;
use crate::renderer::commands::Commands;
use crate::renderer::opengl::OpenGL;
use crate::Vec2;

#[derive(Clone)]
pub struct Settings {
    pub viewport_size: Vec2,
}

pub struct Renderer {
    pub settings: Settings,
    pub backend: Box<dyn Backend>,
}

pub trait Backend {
    fn execute_commands(&mut self, commands: Commands);
}

impl Renderer {
    pub fn opengl(settings: Settings, window: &Window, video_subsystem: &VideoSubsystem) -> Self {
        Self {
            settings: settings.clone(),
            backend: Box::new(OpenGL::new(settings, window, video_subsystem)),
        }
    }

    pub fn begin_frame(&mut self) -> Commands {
        Commands::new()
    }

    pub fn end_frame(&mut self, commands: Commands) {
        self.backend.execute_commands(commands);
    }
}