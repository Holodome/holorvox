use sdl2::video::{GLContext, Window};

pub struct App {
    ctx: sdl2::Sdl,
    vs: sdl2::VideoSubsystem,
    win: Window,
    gl_ctx: GLContext,
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

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        Self {
            ctx: sdl,
            vs: video_subsystem,
            win: window,
            gl_ctx: gl_context,
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

            unsafe {
                gl::ClearColor(0.3, 0.3, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.win.gl_swap_window();
        }
    }
}