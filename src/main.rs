use crate::app::App;

mod app;
mod renderer;
mod assets;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Vec4 = cgmath::Vector4<f32>;
type Mat4 = cgmath::Matrix4<f32>;

fn main() {
    let mut app = App::new();
    app.run();
}