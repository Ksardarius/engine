#![feature(offset_of_enum, offset_of_nested)]

pub mod camera;
pub mod model;
pub mod scene;
pub mod shader;

extern crate glfw;
extern crate image;
extern crate nalgebra_glm as glm;
extern crate russimp;
extern crate tobj;
extern crate fbxcel_dom;

use std::time::Instant;

use crate::{model::Model, shader::Shader};
use camera::Camera;
use glad_gl::gl;
use glfw::{Action, Context, CursorMode, Key, PWindow, WindowEvent};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

static mut first_mouse: bool = false;
static mut last_x: f32 = SCR_WIDTH as f32 / 2.0f32;
static mut last_y: f32 = SCR_HEIGHT as f32 / 2.0f32;

static mut blinn: bool = false;

const MS_PER_UPDATE: f64 = 1.0f64 / 120.0f64;

const dir: &str = "/Users/morlovs/Projects/rust/engine/game/resources/";

use glm::vec3;
// use protobuf::text_format;

// pub mod snazzy {
//     include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
// }

// use snazzy::items;

// pub fn create_large_shirt(color: String) -> items::Shirt {
//     let mut o = items::oneof_example::MessageWithOneof::default();
//     o.not_part_of_oneof = Some("Ttttext".to_string());
//     o.set_first_oneof_field("First one of".to_string());

//     let mut o2 = items::oneof_example::MessageWithOneof::default();
//     o2.not_part_of_oneof = Some("Ttttext".to_string());
//     o2.set_second_oneof_field("Second one of".to_string());

//     let mut ob = items::OneofExample::default();
//     ob.message = vec![o, o2];

//     let mut shirt = items::Shirt::default();
//     shirt.color = color;
//     shirt.size = items::shirt::Size::LARGE.into();
//     // shirt.write_to_bytes();
//     // shirt.set_size(items::shirt::Size::Large);
//     // let a = FileDescriptor::new_dynamic(proto, dependencies)
//     //shirt.

//     //shirt.write_to_writer(w)
//     // let mut text = String::default();

//     let text = text_format::print_to_string_pretty(&ob);
//     println!("Shirt t: {:?}", text);

//     let no = text_format::parse_from_str::<items::OneofExample>(&text).unwrap();
//     println!("Shirt t 2: {:?}", no);

//     shirt
// }

fn main() {
    let scene = scene::config::scene::Scene::load_scene(format!("{dir}scene.txtpb").as_str());

    println!("!!!! Scene loaded: {:?}", scene);

    let mut previous = Instant::now();
    let mut lag = 0.0;

    // let point_light_positions = [
    //     glm::vec3(0.7f32, 0.2f32, 2.0f32),
    //     glm::vec3(2.3f32, -3.3f32, -4.0f32),
    //     glm::vec3(-4.0f32, 2.0f32, -12.0f32),
    //     glm::vec3(0.0f32, 0.0f32, -3.0f32),
    // ];

    // let cube_positions = [
    //     glm::vec3(0.0f32, 0.0f32, 0.0f32),
    //     glm::vec3(2.0f32, 5.0f32, -15.0f32),
    //     glm::vec3(-1.5f32, -2.2f32, -2.5f32),
    //     glm::vec3(-3.8f32, -2.0f32, -12.3f32),
    //     glm::vec3(2.4f32, -0.4f32, -3.5f32),
    //     glm::vec3(-1.7f32, 3.0f32, -7.5f32),
    //     glm::vec3(1.3f32, -2.0f32, -2.5f32),
    //     glm::vec3(1.5f32, 2.0f32, -2.5f32),
    //     glm::vec3(1.5f32, 0.2f32, -1.5f32),
    //     glm::vec3(-1.3f32, 1.0f32, -1.5f32),
    // ];

    // let scene_camera = scene.camera.into_option().unwrap();

    let mut camera: Camera = Camera::new(
        Some(glm::vec3(
            scene.camera.location.x,
            scene.camera.location.y,
            scene.camera.location.z,
        )),
        scene
            .camera
            .up
            .as_ref()
            .map_or(None, move |d| Some(vec3(d.x, d.y, d.z))),
        scene.camera.yaw,
        scene.camera.pitch,
    );

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "Hello this is window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.make_current();

    // window.set_aspect_ratio(16, 10);

    gl::load(|procname| glfw.get_proc_address_raw(procname));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // let shader = Shader::new(
    //     format!("{dir}color.vs").as_str(),
    //     format!("{dir}color.fs").as_str(),
    // );

    let shader = Shader::new(
        format!("{dir}pbr.vs").as_str(),
        format!("{dir}pbr.fs").as_str(),
    );

    // let light_positions = [
    //     glm::vec3(-10.0, 10.0, 10.0),
    //     glm::vec3(10.0, 10.0, 10.0),
    //     glm::vec3(-10.0, -10.0, 10.0),
    //     glm::vec3(10.0, -10.0, 10.0),
    // ];

    // let light_colors = [
    //     glm::vec3(300.0, 300.0, 300.0),
    //     glm::vec3(300.0, 300.0, 300.0),
    //     glm::vec3(300.0, 300.0, 300.0),
    //     glm::vec3(300.0, 300.0, 300.0),
    // ];

    let nr_rows = 7;
    let nr_columns = 7;
    let spacing = 2.5;

    let projection = glm::perspective(
        f32::to_radians(camera.zoom),
        SCR_WIDTH as f32 / SCR_HEIGHT as f32,
        0.1f32,
        100.0f32,
    );

    shader.use_program();
    shader.set_mat_4("projection", &projection);
    // shader.set_vec_3("albedo", 0.5, 0.0, 0.0);
    // shader.set_float("ao", 1.0);

    // let light_cube_shader = Shader::new(
    //     format!("{dir}light_cube.vs").as_str(),
    //     format!("{dir}light_cube.fs").as_str(),
    // );

    // let our_model = Model::new(format!("{dir}backpack/backpack.obj"));
    let our_model = Model::load(format!("{dir}cube/sphere.obj"), model::ModelType::OBJ);
    let our_model2 = Model::load(format!("{dir}Sketchfab Sword forsale.fbx"), model::ModelType::FBX);
    // let our_model = Model::new(format!("{dir}cube/cube.obj"));
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
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();
            let view = camera.get_view_matrix();
            shader.set_mat_4("view", &view);
            shader.set_vec_3v("camPos", &camera.position);

            // shader.set_int("blinn", blinn as i32);

            // shader.set_vec_3v("viewPos", &camera.position);
            // shader.set_float("material.shininess", 8.0f32);

            // shader.set_vec_3("dirLight.direction", -0.2f32, -1.0f32, -0.3f32);
            // shader.set_vec_3("dirLight.ambient", 0.05f32, 0.05f32, 0.05f32);
            // shader.set_vec_3("dirLight.diffuse", 0.4f32, 0.4f32, 0.4f32); // darken diffuse light a bit
            // shader.set_vec_3("dirLight.specular", 0.5f32, 0.5f32, 0.5f32);

            // shader.set_vec_3v("pointLights[0].position", &point_light_positions[0]);
            // shader.set_vec_3("pointLights[0].ambient", 0.05f32, 0.05f32, 0.05f32);
            // shader.set_vec_3("pointLights[0].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            // shader.set_vec_3("pointLights[0].specular", 1.0f32, 1.0f32, 1.0f32);
            // shader.set_float("pointLights[0].constant", 1.0f32);
            // shader.set_float("pointLights[0].linear", 0.09f32);
            // shader.set_float("pointLights[0].quadratic", 0.032f32);

            // shader.set_vec_3v("pointLights[1].position", &point_light_positions[1]);
            // shader.set_vec_3("pointLights[1].ambient", 0.05f32, 0.05f32, 0.05f32);
            // shader.set_vec_3("pointLights[1].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            // shader.set_vec_3("pointLights[1].specular", 1.0f32, 1.0f32, 1.0f32);
            // shader.set_float("pointLights[1].constant", 1.0f32);
            // shader.set_float("pointLights[1].linear", 0.09f32);
            // shader.set_float("pointLights[1].quadratic", 0.032f32);

            // shader.set_vec_3v("pointLights[2].position", &point_light_positions[2]);
            // shader.set_vec_3("pointLights[2].ambient", 0.05f32, 0.05f32, 0.05f32);
            // shader.set_vec_3("pointLights[2].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            // shader.set_vec_3("pointLights[2].specular", 1.0f32, 1.0f32, 1.0f32);
            // shader.set_float("pointLights[2].constant", 1.0f32);
            // shader.set_float("pointLights[2].linear", 0.09f32);
            // shader.set_float("pointLights[2].quadratic", 0.032f32);

            // shader.set_vec_3v("pointLights[3].position", &point_light_positions[3]);
            // shader.set_vec_3("pointLights[3].ambient", 0.05f32, 0.05f32, 0.05f32);
            // shader.set_vec_3("pointLights[3].diffuse", 0.8f32, 0.8f32, 0.8f32); // darken diffuse light a bit
            // shader.set_vec_3("pointLights[3].specular", 1.0f32, 1.0f32, 1.0f32);
            // shader.set_float("pointLights[3].constant", 1.0f32);
            // shader.set_float("pointLights[3].linear", 0.09f32);
            // shader.set_float("pointLights[3].quadratic", 0.032f32);

            // shader.set_vec_3v("spotLight.position", &camera.position);
            // shader.set_vec_3v("spotLight.direction", &camera.front);
            // shader.set_vec_3("spotLight.ambient", 0.0, 0.0, 0.0);
            // shader.set_vec_3("spotLight.diffuse", 1.0, 1.0, 1.0);
            // shader.set_vec_3("spotLight.specular", 1.0, 1.0, 1.0);
            // shader.set_float("spotLight.constant", 1.0);
            // shader.set_float("spotLight.linear", 0.09);
            // shader.set_float("spotLight.quadratic", 0.032);
            // shader.set_float("spotLight.cutOff", f32::cos(f32::to_radians(12.5)));
            // shader.set_float("spotLight.outerCutOff", f32::cos(f32::to_radians(15.0)));

            for row in 0..nr_rows {
                // shader.set_float("metallic", row as f32 / nr_rows as f32);
                for col in 0..nr_columns {
                    // shader.set_float(
                    //     "roughness",
                    //     glm::clamp_scalar(col as f32 / nr_columns as f32, 0.05, 1.0),
                    // );
                    let mut model = glm::Mat4::identity();
                    model = glm::translate(
                        &model,
                        &glm::vec3(
                            (col as f32 - (nr_columns as f32 / 2.0)) * spacing,
                            (row as f32 - (nr_rows as f32 / 2.0)) * spacing,
                            0.0f32,
                        ),
                    );
                    shader.set_mat_4("model", &model);
                    shader.set_mat_3(
                        "normalMatrix",
                        &glm::transpose(&glm::mat4_to_mat3(&model).try_inverse().unwrap()),
                    );
                    our_model.draw(&shader);
                }
            }

            for i in 0..scene.light.len() {
                shader.set_vec_3v(format!("lightPositions[{i}]").as_str(), &vec3(scene.light[i].location.x, scene.light[i].location.y, scene.light[i].location.z));
                shader.set_vec_3v(format!("lightColors[{i}]").as_str(), &vec3(scene.light[i].color.x, scene.light[i].color.y, scene.light[i].color.z));

                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &vec3(scene.light[i].location.x, scene.light[i].location.y, scene.light[i].location.z));
                model = glm::scale(&model, &glm::vec3(0.5, 0.5, 0.5));
                shader.set_mat_4("model", &model);
                shader.set_mat_3(
                    "normalMatrix",
                    &glm::transpose(&glm::mat4_to_mat3(&model).try_inverse().unwrap()),
                );
                our_model.draw(&shader);
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
        glfw::WindowEvent::Key(Key::B, _, Action::Release, _) => unsafe {
            blinn = !blinn;
        },
        // glfw::WindowEvent::Size(x, y) => {
        //     glfw.
        // }
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
