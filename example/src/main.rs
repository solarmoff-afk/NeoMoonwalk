use moonwalk::{MoonWalk, ObjectId};
use moonwalk_bootstrap::{Application, Runner, WindowSettings};
use glam::{Vec2, Vec3, Vec4};

#[cfg(target_os = "android")]
use log::LevelFilter;

const RECT_COUNT: usize = 100_000;
const RECT_SIZE: f32 = 1.0;
const SPEED_BASE: f32 = 200.0;

struct Bouncer {
    id: ObjectId,
    pos: Vec2,
    vel: Vec2,
    color: Vec4,
    rot_speed: f32,
    angle: f32,
}

struct StressApp {
    bouncers: Vec<Bouncer>,
    screen_size: Vec2,
}

impl StressApp {
    fn new() -> Self {
        Self {
            bouncers: Vec::with_capacity(RECT_COUNT),
            screen_size: Vec2::new(1024.0, 768.0),
        }
    }

    fn pseudo_rand(seed: usize, offset: f32) -> f32 {
        ((seed as f32 * 12.9898 + offset).sin() * 43758.5453).fract()
    }
}

impl Application for StressApp {
    fn on_start(&mut self, mw: &mut MoonWalk, viewport: Vec2) {
        self.screen_size = viewport;
        
        let bg = mw.new_rect();
        mw.config_position(bg, Vec2::ZERO);
        mw.config_size(bg, viewport * 2.0);
        mw.config_color(bg, Vec4::new(0.02, 0.02, 0.05, 1.0));
        mw.set_z_index(bg, 0.0);

        println!("Spawning {} rects...", RECT_COUNT);

        for i in 0..RECT_COUNT {
            let id = mw.new_rect();
            
            let r = Self::pseudo_rand(i, 1.0);
            let g = Self::pseudo_rand(i, 2.0);
            let b = Self::pseudo_rand(i, 3.0);
            
            let start_x = Self::pseudo_rand(i, 4.0) * (viewport.x - RECT_SIZE);
            let start_y = Self::pseudo_rand(i, 5.0) * (viewport.y - RECT_SIZE);
            
            let vel_x = (Self::pseudo_rand(i, 6.0) - 0.5) * 2.0 * SPEED_BASE;
            let vel_y = (Self::pseudo_rand(i, 7.0) - 0.5) * 2.0 * SPEED_BASE;

            let color = Vec4::new(r, g, b, 1.0);
            let size = Vec2::splat(RECT_SIZE + Self::pseudo_rand(i, 8.0) * 10.0); // 20..30px

            mw.config_position(id, Vec2::new(start_x, start_y));
            mw.config_size(id, size);
            mw.config_color(id, color);
            mw.set_rounded(id, Vec4::splat(5.0)); // Небольшое скругление
            mw.set_z_index(id, 0.1 + (i as f32 / RECT_COUNT as f32)); 

            self.bouncers.push(Bouncer {
                id,
                pos: Vec2::new(start_x, start_y),
                vel: Vec2::new(vel_x, vel_y),
                color,
                rot_speed: (Self::pseudo_rand(i, 9.0) - 0.5) * 5.0,
                angle: 0.0,
            });
        }
    }

    fn on_update(&mut self, dt: f32) {
        let w = self.screen_size.x;
        let h = self.screen_size.y;

        for b in &mut self.bouncers {
            b.pos += b.vel * dt;
            b.angle += b.rot_speed * dt;

            let mut hit = false;
            
            if b.pos.x < 0.0 {
                b.pos.x = 0.0;
                b.vel.x = b.vel.x.abs();
                hit = true;
            } else if b.pos.x + RECT_SIZE > w {
                b.pos.x = w - RECT_SIZE;
                b.vel.x = -b.vel.x.abs();
                hit = true;
            }

            if b.pos.y < 0.0 {
                b.pos.y = 0.0;
                b.vel.y = b.vel.y.abs();
                hit = true;
            } else if b.pos.y + RECT_SIZE > h {
                b.pos.y = h - RECT_SIZE;
                b.vel.y = -b.vel.y.abs();
                hit = true;
            }

            if hit {
                b.color = Vec4::new(b.color.y, b.color.z, b.color.x, 1.0); // Swizzle
            }
        }
    }

    fn on_draw(&mut self, mw: &mut MoonWalk) {
        for b in &self.bouncers {
            mw.config_position(b.id, b.pos);
            mw.config_rotation(b.id, b.angle);
            mw.config_color(b.id, b.color);
        }
    }

    fn on_resize(&mut self, mw: &mut MoonWalk, viewport: Vec2) {
        self.screen_size = viewport;
    }
}

#[cfg(not(target_os = "android"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = StressApp::new();
    
    let settings = WindowSettings::new("MoonWalk Stress Test", 1280.0, 720.0)
        .resizable(true);
    
        Runner::run(app, settings)
}

#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(LevelFilter::Info)
    );

    let stress_app = StressApp::new();
    let settings = WindowSettings::new("MoonWalk Android", 0.0, 0.0);
    
    Runner::run(stress_app, settings, app).unwrap();
}

#[cfg(target_os = "android")]
fn main() {}