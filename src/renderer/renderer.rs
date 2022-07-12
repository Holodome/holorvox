use crate::renderer::commands::Commands;

pub struct Settings {

}

pub struct Renderer {
    settings: Settings,
    backend: Box<dyn Backend>
}

pub trait Backend {
    fn execute_commands(&mut self, commands: Commands);
}

impl Renderer {
    pub fn end_frame(&mut self) {

    }
}