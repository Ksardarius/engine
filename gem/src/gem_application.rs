use std::time::Instant;

use log::trace;
use res_cache::HandleResCache;
use script_manager::HandleScriptManager;
use strings_cache::HandleStringCache;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub mod game_actors;
pub mod options;
pub mod process_management;
pub mod strings_cache;

struct App<U> {
    window: Option<Window>,
    last_redraw: Instant,
    update_callback: U,
}

impl<U> ApplicationHandler for App<U>
where
    U: FnMut(f64),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // let w = event_loop.create_window(Window::default_attributes()).unwrap();
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let current = Instant::now();
                let elapsed = current.duration_since(self.last_redraw).as_secs_f64();

                trace!("Redraw {elapsed}");
                (self.update_callback)(elapsed);
                self.last_redraw = current;

                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub trait HandleWindow {
    fn on_update(&mut self, delta_s: f64);

    fn create_window(&mut self, x: i32, y: i32) -> bool {
        let event_loop = EventLoop::new().unwrap();
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = App {
            window: None,
            last_redraw: Instant::now(),
            update_callback: |delta_s: f64|self.on_update(delta_s)
        };
        event_loop.run_app(&mut app);

        true
    }
}

// Application layer base
pub trait GemApplication:
    HandleResCache + HandleStringCache + HandleScriptManager + HandleWindow
{
    fn init_instance(&mut self, x: i32, y: i32, localization_path: &str) -> bool {
        self.is_only_one_instance();
        self.check_storage();
        self.check_memory();
        self.read_cpu_speed();

        HandleResCache::init(self);
        HandleStringCache::load_strings(self, localization_path);
        HandleScriptManager::load(self);

        self.create_game_and_view();
        self.create_window(x, y);

        true
    }

    fn create_game_and_view(&mut self) -> bool;

    fn is_only_one_instance(&self) -> bool {
        true
    }

    fn check_storage(&self) -> bool {
        true
    }

    fn check_memory(&self) -> bool {
        true
    }

    fn read_cpu_speed(&self) -> bool {
        true
    }

    fn get_exit_code(&self) -> usize {
        1
    }
}
