use sdl2::video::{GLContext, Window};
use crate::renderer::renderer;
use crate::renderer::renderer::Renderer;
use crate::{Vec2, Vec4};

pub struct App {
    ctx: sdl2::Sdl,
    vs: sdl2::VideoSubsystem,
    win: Window,
    renderer: Renderer
}

impl App {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Game", 900, 700)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let renderer = Renderer::opengl(renderer::Settings {
            viewport_size: Vec2::new(900.0, 700.0)
        }, &window, &video_subsystem);

        Self {
            ctx: sdl,
            vs: video_subsystem,
            win: window,
            renderer
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.ctx.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            let mut commands = self.renderer.begin_frame();
            commands.clear(Vec4::new(1.0, 0.0, 1.0, 1.0));
            self.renderer.end_frame(commands);

            self.win.gl_swap_window();
        }
    }
}