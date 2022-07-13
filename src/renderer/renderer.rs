use sdl2::video::Window;
use sdl2::VideoSubsystem;
use crate::renderer::commands::Commands;
use crate::renderer::opengl::OpenGL;
use crate::Vec2;

#[derive(Clone)]
pub struct Settings {
    pub viewport_size: Vec2,
    pub max_vertex_count: usize,
    pub max_index_count: usize,
}

impl Settings {
    pub fn new(viewport_size: Vec2) -> SettingsBuilder {
        SettingsBuilder::new(viewport_size)
    }
}

pub struct SettingsBuilder {
    viewport_size: Vec2,
    max_vertex_count: Option<usize>,
    max_index_count: Option<usize>,
}

impl SettingsBuilder {
    pub fn new(viewport_size: Vec2) -> SettingsBuilder {
        Self {
            viewport_size,
            max_vertex_count: None,
            max_index_count: None,
        }
    }

    pub fn build(self) -> Settings {
        const MAX_VERTEX_COUNT: usize = 1 << 16;
        const MAX_INDEX_COUNT: usize = MAX_VERTEX_COUNT * 2 / 3;

        Settings {
            viewport_size: self.viewport_size,
            max_vertex_count: self.max_vertex_count.unwrap_or(MAX_VERTEX_COUNT),
            max_index_count: self.max_index_count.unwrap_or(MAX_INDEX_COUNT),
        }
    }
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