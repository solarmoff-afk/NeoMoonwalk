use std::time::Instant;
use glam::{Vec2, Vec4};
use moonwalk::MoonWalk;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
    dpi::LogicalSize,
};

use crate::app::Application;
use crate::window::WindowSettings;

struct AppState {
    window: &'static Window,
    moonwalk: MoonWalk,
    last_frame_time: Instant,
}

struct AppRunner<A> {
    app: A,
    settings: WindowSettings,
    state: Option<AppState>,
}

impl<A: Application> ApplicationHandler for AppRunner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let mut attributes = Window::default_attributes()
            .with_title(&self.settings.title)
            .with_inner_size(LogicalSize::new(self.settings.initial_size.x, self.settings.initial_size.y))
            .with_resizable(self.settings.resizable)
            .with_transparent(self.settings.transparent)
            .with_decorations(self.settings.decorated);

        if let Some(min) = self.settings.min_size {
            attributes = attributes.with_min_inner_size(LogicalSize::new(min.x, min.y));
        }
        
        if let Some(max) = self.settings.max_size {
            attributes = attributes.with_max_inner_size(LogicalSize::new(max.x, max.y));
        }

        let window = event_loop.create_window(attributes).expect("Failed to create window");
        let static_window: &'static Window = Box::leak(Box::new(window));

        let initial_size = static_window.inner_size();

        let mut moonwalk = MoonWalk::new(
            static_window, 
            initial_size.width, 
            initial_size.height
        ).expect("Failed to init MoonWalk");
        
        let scale_factor = static_window.scale_factor();
        let logical_size = initial_size.to_logical::<f32>(scale_factor);
        
        moonwalk.set_viewport(initial_size.width, initial_size.height);

        self.app.on_start(&mut moonwalk, Vec2::new(logical_size.width, logical_size.height));

        self.state = Some(AppState {
            window: static_window,
            moonwalk,
            last_frame_time: Instant::now(),
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let state = match self.state.as_mut() {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                self.app.on_exit();
                event_loop.exit();
            },
            
            WindowEvent::Resized(physical_size) => {
                state.moonwalk.set_viewport(physical_size.width, physical_size.height);
                
                let scale = state.window.scale_factor();
                let logical = physical_size.to_logical::<f32>(scale);
                
                self.app.on_resize(&mut state.moonwalk, Vec2::new(logical.width, logical.height));
            },

            WindowEvent::RedrawRequested => {
                state.window.request_redraw();

                let now = Instant::now();
                let delta_time = now.duration_since(state.last_frame_time).as_secs_f32();
                state.last_frame_time = now;

                self.app.on_update(delta_time);
                self.app.on_draw(&mut state.moonwalk);
                
                if let Err(e) = state.moonwalk.render_frame(Vec4::new(0.1, 0.1, 0.1, 1.0)) {
                    eprintln!("Render error: {}", e);
                }
            },

            _ => {}
        }
    }
}

pub struct Runner;

impl Runner {
    pub fn run<A: Application + 'static>(app: A, settings: WindowSettings) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut runner = AppRunner {
            app,
            settings,
            state: None,
        };

        event_loop.run_app(&mut runner)?;

        Ok(())
    }
}