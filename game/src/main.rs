#![feature(offset_of_enum, offset_of_nested)]

pub mod camera;
pub mod mesh;
pub mod model;
// pub mod renderer;
pub mod shader;

extern crate glfw;
extern crate image;
extern crate nalgebra_glm as glm;
extern crate russimp;

use std::time::Instant;

use crate::{model::Model, shader::Shader};
use camera::Camera;
use glad_gl::gl;
use glfw::{Action, Context, CursorMode, Key, PWindow, WindowEvent};
// use renderer::Renderer;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

static mut first_mouse: bool = false;
static mut last_x: f32 = SCR_WIDTH as f32 / 2.0f32;
static mut last_y: f32 = SCR_HEIGHT as f32 / 2.0f32;

const MS_PER_UPDATE: f64 = 1.0f64 / 120.0f64;

const dir: &str = "/Users/morlovs/Projects/rust/engine/game/resources/";

fn main() {
    let mut previous = Instant::now();
    let mut lag = 0.0;

    let point_light_positions = [
        glm::vec3(0.7f32, 0.2f32, 2.0f32),
        glm::vec3(2.3f32, -3.3f32, -4.0f32),
        glm::vec3(-4.0f32, 2.0f32, -12.0f32),
        glm::vec3(0.0f32, 0.0f32, -3.0f32),
    ];

    let cube_positions = [
        glm::vec3(0.0f32, 0.0f32, 0.0f32),
        glm::vec3(2.0f32, 5.0f32, -15.0f32),
        glm::vec3(-1.5f32, -2.2f32, -2.5f32),
        glm::vec3(-3.8f32, -2.0f32, -12.3f32),
        glm::vec3(2.4f32, -0.4f32, -3.5f32),
        glm::vec3(-1.7f32, 3.0f32, -7.5f32),
        glm::vec3(1.3f32, -2.0f32, -2.5f32),
        glm::vec3(1.5f32, 2.0f32, -2.5f32),
        glm::vec3(1.5f32, 0.2f32, -1.5f32),
        glm::vec3(-1.3f32, 1.0f32, -1.5f32),
    ];

    let mut camera: Camera = Camera::new(Some(glm::vec3(0.0, 0.0, 3.0)), None, None, None);

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 800, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.make_current();

    gl::load(|procname| glfw.get_proc_address_raw(procname));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let shader = Shader::new(
        format!("{dir}color.vs").as_str(),
        format!("{dir}color.fs").as_str(),
    );

    // let light_cube_shader = Shader::new(
    //     format!("{dir}light_cube.vs").as_str(),
    //     format!("{dir}light_cube.fs").as_str(),
    // );

    let our_model = Model::new(format!("{dir}backpack/backpack.obj"));
    // println!("Model t: {:?}", our_model);

    // let mut renderer = Renderer::init(camera);

    while !window.should_close() {
        let current = Instant::now();
        let elapsed = current.duration_since(previous).as_secs_f64();
        previous = current;
        lag += elapsed;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut camera, elapsed as f32);
        }

        while lag >= MS_PER_UPDATE {
            // UPDATE
            lag -= MS_PER_UPDATE;
        }

        // render
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.05, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

            shader.set_vec_3v("viewPos", &camera.position);
            shader.set_float("material.shininess", 32.0f32);

            shader.set_vec_3("dirLight.direction", -0.2f32, -1.0f32, -0.3f32);
            shader.set_vec_3("dirLight.ambient", 0.05f32, 0.05f32, 0.05f32);
            shader.set_vec_3("dirLight.diffuse", 0.4f32, 0.4f32, 0.4f32); // darken diffuse light a bit
            shader.set_vec_3("dirLight.specular", 0.5f32, 0.5f32, 0.5f32);

            shader.set_vec_3v("pointLights[0].position", &point_light_positions[0]);
            shader.set_vec_3("pointLights[0].ambient", 0.05f32, 0.05f32, 0.05f32);
            shader.set_vec_3("pointLights[0].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            shader.set_vec_3("pointLights[0].specular", 1.0f32, 1.0f32, 1.0f32);
            shader.set_float("pointLights[0].constant", 1.0f32);
            shader.set_float("pointLights[0].linear", 0.09f32);
            shader.set_float("pointLights[0].quadratic", 0.032f32);

            shader.set_vec_3v("pointLights[1].position", &point_light_positions[1]);
            shader.set_vec_3("pointLights[1].ambient", 0.05f32, 0.05f32, 0.05f32);
            shader.set_vec_3("pointLights[1].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            shader.set_vec_3("pointLights[1].specular", 1.0f32, 1.0f32, 1.0f32);
            shader.set_float("pointLights[1].constant", 1.0f32);
            shader.set_float("pointLights[1].linear", 0.09f32);
            shader.set_float("pointLights[1].quadratic", 0.032f32);

            shader.set_vec_3v("pointLights[2].position", &point_light_positions[2]);
            shader.set_vec_3("pointLights[2].ambient", 0.05f32, 0.05f32, 0.05f32);
            shader.set_vec_3("pointLights[2].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            shader.set_vec_3("pointLights[2].specular", 1.0f32, 1.0f32, 1.0f32);
            shader.set_float("pointLights[2].constant", 1.0f32);
            shader.set_float("pointLights[2].linear", 0.09f32);
            shader.set_float("pointLights[2].quadratic", 0.032f32);

            shader.set_vec_3v("pointLights[3].position", &point_light_positions[3]);
            shader.set_vec_3("pointLights[3].ambient", 0.05f32, 0.05f32, 0.05f32);
            shader.set_vec_3("pointLights[3].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            shader.set_vec_3("pointLights[3].specular", 1.0f32, 1.0f32, 1.0f32);
            shader.set_float("pointLights[3].constant", 1.0f32);
            shader.set_float("pointLights[3].linear", 0.09f32);
            shader.set_float("pointLights[3].quadratic", 0.032f32);

            shader.set_vec_3v("spotLight.position", &camera.position);
            shader.set_vec_3v("spotLight.direction", &camera.front);
            shader.set_vec_3("spotLight.ambient", 0.0, 0.0, 0.0);
            shader.set_vec_3("spotLight.diffuse", 1.0, 1.0, 1.0);
            shader.set_vec_3("spotLight.specular", 1.0, 1.0, 1.0);
            shader.set_float("spotLight.constant", 1.0);
            shader.set_float("spotLight.linear", 0.09);
            shader.set_float("spotLight.quadratic", 0.032);
            shader.set_float("spotLight.cutOff", f32::cos(f32::to_radians(12.5)));
            shader.set_float("spotLight.outerCutOff", f32::cos(f32::to_radians(15.0)));

            let projection = glm::perspective(
                f32::to_radians(camera.zoom),
                800.0f32 / 800.0f32,
                0.1f32,
                100.0f32,
            );
            let view = camera.get_view_matrix();

            shader.set_mat_4("projection", &projection);
            shader.set_mat_4("view", &view);

            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, diffuse_map);

            // gl::ActiveTexture(gl::TEXTURE1);
            // gl::BindTexture(gl::TEXTURE_2D, specular_map);

            // gl::BindVertexArray(vao_id);

            for i in 0..1 {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &cube_positions[i]);
                let angle = 20.0f32 * i as f32;
                model = glm::rotate(&model, f32::to_radians(angle), &glm::vec3(1.0, 0.3, 0.5));

                // model = glm::rotate(&model, self.glfw.get_time() as f32, &glm::vec3(1.0, 0.0, 0.0));
                shader.set_mat_4("model", &model);
                our_model.draw(&shader);
                // gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // light_cube_shader.use_program();
            // light_cube_shader.set_mat_4("projection", &projection);
            // light_cube_shader.set_mat_4("view", &view);

            // gl::BindVertexArray(light_vao);

            // for i in 0..4 {
            //     let mut model = glm::Mat4::identity();
            //     model = glm::translate(&model, &point_light_positions[i]);
            //     model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));

            //     light_cube_shader.set_mat_4("model", &model);
            //     gl::DrawArrays(gl::TRIANGLES, 0, 36);
            // }
        }
        // draw();

        window.swap_buffers();
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

        glfw::WindowEvent::Scroll(_, ypos) => {
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
