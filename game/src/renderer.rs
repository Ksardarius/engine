use std::{
    mem::{size_of, size_of_val}, os::raw::c_void
};

use glad_gl::gl;
use glfw::{Context, CursorMode, PWindow, WindowEvent};
use image::EncodableLayout;

use crate::{camera::Camera, shader::Shader};

// const BOX_VERTICES: [gl::GLfloat; 32] = [
//     // positions          // colors           // texture coords
//     0.5f32,  0.5f32, 0.0f32,   1.0f32, 0.0f32, 0.0f32,   1.0f32, 1.0f32,   // top right
//     0.5f32, -0.5f32, 0.0f32,   0.0f32, 1.0f32, 0.0f32,   1.0f32, 0.0f32,   // bottom right
//    -0.5f32, -0.5f32, 0.0f32,   0.0f32, 0.0f32, 1.0f32,   0.0f32, 0.0f32,   // bottom left
//    -0.5f32,  0.5f32, 0.0f32,   1.0f32, 1.0f32, 0.0f32,   0.0f32, 1.0f32    // top left 
// ];

const VERTICES: [f32; 288] = [
    // positions          // normals           // texture coords
    -0.5f32, -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32, 0.0f32,
     0.5f32, -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32, 1.0f32,
     0.5f32,  0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  1.0f32, 1.0f32,
    -0.5f32,  0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32, -0.5f32,  0.0f32,  0.0f32, -1.0f32,  0.0f32, 0.0f32,

    -0.5f32, -0.5f32,  0.5f32,  0.0f32,  0.0f32, 1.0f32,   0.0f32, 0.0f32,
     0.5f32, -0.5f32,  0.5f32,  0.0f32,  0.0f32, 1.0f32,   1.0f32, 0.0f32,
     0.5f32,  0.5f32,  0.5f32,  0.0f32,  0.0f32, 1.0f32,   1.0f32, 1.0f32,
     0.5f32,  0.5f32,  0.5f32,  0.0f32,  0.0f32, 1.0f32,   1.0f32, 1.0f32,
    -0.5f32,  0.5f32,  0.5f32,  0.0f32,  0.0f32, 1.0f32,   0.0f32, 1.0f32,
    -0.5f32, -0.5f32,  0.5f32,  0.0f32,  0.0f32, 1.0f32,   0.0f32, 0.0f32,

    -0.5f32,  0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32, 0.0f32,
    -0.5f32,  0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32, 1.0f32,
    -0.5f32, -0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32, -0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  0.0f32, 0.0f32,
    -0.5f32,  0.5f32,  0.5f32, -1.0f32,  0.0f32,  0.0f32,  1.0f32, 0.0f32,

     0.5f32,  0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32, 1.0f32,
     0.5f32, -0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32, 1.0f32,
     0.5f32, -0.5f32, -0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32, 1.0f32,
     0.5f32, -0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  0.0f32, 0.0f32,
     0.5f32,  0.5f32,  0.5f32,  1.0f32,  0.0f32,  0.0f32,  1.0f32, 0.0f32,

    -0.5f32, -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32, 1.0f32,
     0.5f32, -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32, 1.0f32,
     0.5f32, -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32, 0.0f32,
     0.5f32, -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  1.0f32, 0.0f32,
    -0.5f32, -0.5f32,  0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32, 0.0f32,
    -0.5f32, -0.5f32, -0.5f32,  0.0f32, -1.0f32,  0.0f32,  0.0f32, 1.0f32,

    -0.5f32,  0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32, 1.0f32,
     0.5f32,  0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32, 1.0f32,
     0.5f32,  0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  1.0f32, 0.0f32,
    -0.5f32,  0.5f32,  0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32, 0.0f32,
    -0.5f32,  0.5f32, -0.5f32,  0.0f32,  1.0f32,  0.0f32,  0.0f32, 1.0f32
];

// const INDICES: [i32; 6] = [  // note that we start from 0!
//     0, 1, 3,   // first triangle
//     1, 2, 3    // second triangle
// ]; 

pub struct Renderer {
    pub glfw: glfw::Glfw,
    pub window: PWindow,
    pub events: glfw::GlfwReceiver<(f64, WindowEvent)>,

    pub camera: Camera,

    vao_id: gl::GLuint,
    vbo_id: gl::GLuint,

    light_vao: gl::GLuint,

    shader: Shader,
    light_shader: Shader,

    diffuse_map: u32,
    specular_map: u32,

    // model: glm::Mat4,
    // view: glm::Mat4,
    // projection: glm::Mat4,
}

const dir: &str = "/Users/morlovs/Projects/rust/engine/game/resources/";

impl Renderer {
    pub fn init(camera: Camera) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(CursorMode::Disabled);
        window.make_current();

        gl::load(|procname| glfw.get_proc_address_raw(procname));

        let shader = Shader::new(
             format!("{dir}color.vs").as_str(),
             format!("{dir}color.fs").as_str(),
        );

        let light_cube_shader = Shader::new(
            format!("{dir}light_cube.vs").as_str(),
            format!("{dir}light_cube.fs").as_str(),
        );

        let mut renderer = Self {
            glfw: glfw,
            window: window,
            events: events,

            camera: camera,
            vao_id: 0,
            vbo_id: 0,

            light_vao: 0,

            shader,
            light_shader: light_cube_shader,

            diffuse_map: 0,
            specular_map: 0,
        };

        // let mut renderer = Self::new(camera);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            
            gl::GenVertexArrays(1,  &mut renderer.vao_id as *mut gl::GLuint);
            gl::GenBuffers(1, &mut  renderer.vbo_id as *mut gl::GLuint);

            gl::BindVertexArray(renderer.vao_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, renderer.vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES).try_into().unwrap(),
                &VERTICES as *const _ as *const c_void,
                gl::STATIC_DRAW,
            );

            let a_pos: u32 = renderer.shader.get_attrib_location("aPos");
            gl::EnableVertexAttribArray(a_pos);
            gl::VertexAttribPointer(
                a_pos,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                std::ptr::null::<c_void>(),
            );

            let a_normal: u32 = renderer.shader.get_attrib_location("aNormal");
            gl::EnableVertexAttribArray(a_normal);
            gl::VertexAttribPointer(
                a_normal,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                std::ptr::null::<c_void>().offset(3 * size_of::<gl::GLfloat>() as isize),
            );

            let a_text_coord: u32 = renderer.shader.get_attrib_location("aTexCoords");
            gl::EnableVertexAttribArray(a_text_coord);
            gl::VertexAttribPointer(
                a_text_coord,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                std::ptr::null::<c_void>().offset(6 * size_of::<gl::GLfloat>() as isize),
            );


            gl::GenVertexArrays(1,  &mut renderer.light_vao as *mut gl::GLuint);
            gl::BindVertexArray(renderer.light_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, renderer.vbo_id);

            let a_pos: u32 = renderer.light_shader.get_attrib_location("aPos");

            gl::EnableVertexAttribArray(a_pos);
            gl::VertexAttribPointer(
                a_pos,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * size_of::<gl::GLfloat>() as i32,
                std::ptr::null::<c_void>(),
            );


            // textures
            renderer.diffuse_map = renderer.load_texture(format!("{dir}container.png").as_str()).unwrap();
            renderer.specular_map = renderer.load_texture(format!("{dir}container_specular.png").as_str()).unwrap();
            renderer.shader.use_program();
            renderer.shader.set_int("material.diffuse", 0);
            renderer.shader.set_int("material.specular", 1);

            // gl::EnableVertexAttribArray(a_tex_coord);
            // gl::VertexAttribPointer(
            //     a_tex_coord,
            //     2,
            //     gl::FLOAT,
            //     gl::FALSE,
            //     5 * size_of::<gl::GLfloat>() as i32,
            //     std::ptr::null::<c_void>().offset(3 * size_of::<gl::GLfloat>() as isize),
            // );

            // renderer.load_textures();


            println!("vao: {:?}, vbo: {:?}, vpoint: {:?}, light: {:?}", renderer.vao_id, renderer.vbo_id, a_pos, renderer.light_vao);
        }

        // let shader = renderer.init();
        renderer
    }

    pub fn draw(&self) {
        let light_pos: glm::Vec3 = glm::vec3(1.2f32, 1.0f32, 2.0f32);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.shader.use_program();
            self.shader.set_vec_3("objectColor", 1.0f32, 0.5f32, 0.31f32);
            self.shader.set_vec_3("lightColor", 1.0f32, 1.0f32, 1.0f32);
            self.shader.set_vec_3v("lightPos", &light_pos);
            self.shader.set_vec_3v("viewPos", &self.camera.position);

            // self.shader.set_vec_3("material.ambient", 1.0f32, 0.5f32, 0.31f32);
            // self.shader.set_vec_3("material.diffuse", 1.0f32, 0.5f32, 0.31f32);
            self.shader.set_vec_3("material.specular", 0.5f32, 0.5f32, 0.5f32);
            self.shader.set_float("material.shininess", 32.0f32);

            self.shader.set_vec_3("light.ambient",  0.2f32, 0.2f32, 0.2f32);
            self.shader.set_vec_3("light.diffuse",  0.5f32, 0.5f32, 0.5f32); // darken diffuse light a bit
            self.shader.set_vec_3("light.specular", 1.0f32, 1.0f32, 1.0f32); 

            let projection = glm::perspective(45.0f32.to_radians(), 800.0f32 / 600.0f32, 0.1f32, 100.0f32);
            let view = self.camera.get_view_matrix();
            
            self.shader.set_mat_4("projection", &projection);
            self.shader.set_mat_4("view", &view);

            let mut model = glm::Mat4::identity();
            model = glm::rotate(&model, self.glfw.get_time() as f32, &glm::vec3(1.0, 0.0, 0.0));
            self.shader.set_mat_4("model", &model);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.diffuse_map);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.specular_map);

            gl::BindVertexArray(self.vao_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);


            self.light_shader.use_program();
            self.light_shader.set_mat_4("projection", &projection);
            self.light_shader.set_mat_4("view", &view);

            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &light_pos);
            model = glm::scale(&model, &glm::vec3(0.2f32, 0.2f32, 0.2f32));
            self.light_shader.set_mat_4("model", &model);

            gl::BindVertexArray(self.light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

        }
    }

    fn load_texture(&mut self, path: &str) -> Result<gl::GLuint, Box<dyn std::error::Error>> {    
        unsafe {
            let mut texture_id: gl::GLuint = 0;

            gl::GenTextures(1, &mut texture_id as *mut gl::GLuint);

            // let img = image::open(path)?
            //     //.unwrap()
            //     .flipv()
            //     .into_rgba8();

            let img = image::open(path)?;
            let channel_count = img.color().channel_count();

            let format = match channel_count {
                1 => gl::RED,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => gl::RGBA
            };

            
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.flipv().as_bytes().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            Ok(texture_id)
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        println!("Update t: {:?} dt: {:?}", "fdsfds", "vcxvc");

        unsafe {
            gl::DeleteVertexArrays(1, self.vao_id as *const gl::GLuint);
            gl::DeleteBuffers(1, self.vbo_id as *const gl::GLuint);
        }
    }
}
