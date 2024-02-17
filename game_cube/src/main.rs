pub mod camera;
pub mod renderer;
pub mod shader;

extern crate glfw;
extern crate image;
extern crate nalgebra_glm as glm;

use std::{borrow::BorrowMut, cell::{Cell, RefCell}, rc::Rc, time::Instant};

use camera::Camera;
use glfw::{Action, Context, Key};
use renderer::Renderer;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

static mut first_mouse: bool = false;
static mut last_x: f32 = SCR_WIDTH as f32 / 2.0f32;
static mut last_y: f32 = SCR_HEIGHT as f32 / 2.0f32;

const MS_PER_UPDATE: f64 = 1.0f64 / 60.0f64;

fn main() {
    let mut previous = Instant::now();
    let mut lag = 0.0;

    let camera: Camera = Camera::new(Some(glm::vec3(0.0, 0.0, 3.0)), None, None, None);


    let (mut renderer, shader) = Renderer::init(camera);

    while !renderer.window.should_close() {
        let current = Instant::now();
        let elapsed = current.duration_since(previous).as_secs_f64();
        previous = current;
        lag += elapsed;

        renderer.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&renderer.events) {
            handle_window_event(
                &mut renderer.window,
                event,
                &mut renderer.camera,
                elapsed as f32,
            );
        }

        while lag >= MS_PER_UPDATE {
            // UPDATE
            lag -= MS_PER_UPDATE;
        }

        // render
        renderer.draw(&shader);
        renderer.window.swap_buffers();
    }
}

fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    camera: &mut Camera,
    frame_time: f32,
) {
    match event {
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            let (x, y) = mouse_callback(xpos as f32, ypos as f32);
            camera.process_mouse_movement(x, y, true);
        }

        glfw::WindowEvent::Scroll(_, ypos ) => {
            camera.process_mouse_scroll(ypos as f32);
        }

        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

        glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
            camera.process_keyboard(camera::CameraMovement::FORWARD, frame_time)
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
            camera.process_keyboard(camera::CameraMovement::BACKWARD, frame_time)
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
            camera.process_keyboard(camera::CameraMovement::LEFT, frame_time)
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
            camera.process_keyboard(camera::CameraMovement::RIGHT, frame_time)
        }

        _ => {}
    }
}

fn mouse_callback(xpos: f32, ypos: f32) -> (f32, f32) {
    unsafe {
        if first_mouse {
            last_x = xpos;
            last_y = ypos;
            first_mouse = false;
        }

        let xoffset = xpos - last_x;
        let yoffset = last_y - ypos;

        last_x = xpos;
        last_y = ypos;

        (xoffset, yoffset)
    }
}
