use moonwalk::MoonWalk;

pub use moonwalk_bootstrap::{Application, Runner, WindowSettings};
pub use glam::{Vec2, Vec3, Vec4, Quat, Mat4};

struct ExampleApp;

impl Application for ExampleApp {
    fn on_start(&mut self, _mw: &mut MoonWalk, _viewport: Vec2) {}

    fn on_update(&mut self, _dt: f32) {}

    fn on_draw(&mut self, _mw: &mut MoonWalk) {}

    fn on_resize(&mut self, _mw: &mut MoonWalk, _viewport: Vec2) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = ExampleApp;
    let settings = WindowSettings::new("MoonWalk Bridge", 1024.0, 768.0)
        .resizable(true);
    
    Runner::run(app, settings)
}