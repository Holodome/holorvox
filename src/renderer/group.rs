use crate::assets::Assets;
use crate::renderer::commands::{CommandQuad, Commands};
use crate::{Vec2, Vec3, Vec4};
use crate::renderer::misc::{Quad, Setup};

pub struct RenderGroup<'a> {
    assets: &'a mut Assets,
    commands: &'a mut Commands,
}


impl<'a> RenderGroup<'a> {
    pub fn new(assets: &'a mut Assets,
               commands: &'a mut Commands, setup: Setup) -> Self {
        commands.setup(setup);
        Self {
            assets,
            commands,
        }
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.commands.quad(quad);
    }
}