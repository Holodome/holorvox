use sdl2::video::{GLContext, GLProfile, Window};
use crate::renderer::renderer;
use crate::renderer::renderer::Renderer;
use crate::{Vec2, Vec3, Vec4};
use crate::renderer::misc::Quad;

pub struct App {
    ctx: sdl2::Sdl,
    vs: sdl2::VideoSubsystem,
    win: Window,
    renderer: Renderer,
    gl_ctx: GLContext
}

impl App {
    pub fn new() -> Self {
        let sdl = sdl2::init().expect("Failed to initialize opengl");
        let video_subsystem = sdl.video().expect("Failed to initialize video subsystem");

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("Game", 900, 700)
            .opengl()
            .resizable()
            .build()
            .expect("Failed to create window");

        let gl_ctx = window.gl_create_context().expect("Failed to create opengl context");
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let renderer = Renderer::opengl(renderer::Settings::new(
            Vec2::new(900.0, 700.0)
        ).build());

        Self {
            ctx: sdl,
            vs: video_subsystem,
            win: window,
            renderer,
            gl_ctx
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
            commands.quad(Quad::v4c4(Vec3::new(-1.0, -1.0, 0.0),
                                    Vec3::new(-1.0, 0.0, 0.0),
                                    Vec3::new(0.0, -1.0, 0.0),
                                    Vec3::new(0.0, 0.0, 0.0),
                                    Vec4::new(0.0, 0.0, 1.0, 1.0),
                                     Vec4::new(0.0, 1.0, 0.0, 1.0),
                                     Vec4::new(1.0, 0.0, 0.0, 1.0),
                                     Vec4::new(0.0, 1.0, 1.0, 1.0),

            ));
            self.renderer.end_frame(commands);

            self.win.gl_swap_window();
        }
    }
}