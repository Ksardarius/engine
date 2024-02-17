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

const VERTICES: [f32; 180] = [
    -0.5f32, -0.5f32, -0.5f32,  0.0f32, 0.0f32,
     0.5f32, -0.5f32, -0.5f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32, -0.5f32,  1.0f32, 1.0f32,
     0.5f32,  0.5f32, -0.5f32,  1.0f32, 1.0f32,
    -0.5f32,  0.5f32, -0.5f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32, -0.5f32,  0.0f32, 0.0f32,

    -0.5f32, -0.5f32,  0.5f32,  0.0f32, 0.0f32,
     0.5f32, -0.5f32,  0.5f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32,  0.5f32,  1.0f32, 1.0f32,
     0.5f32,  0.5f32,  0.5f32,  1.0f32, 1.0f32,
    -0.5f32,  0.5f32,  0.5f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32,  0.5f32,  0.0f32, 0.0f32,

    -0.5f32,  0.5f32,  0.5f32,  1.0f32, 0.0f32,
    -0.5f32,  0.5f32, -0.5f32,  1.0f32, 1.0f32,
    -0.5f32, -0.5f32, -0.5f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32, -0.5f32,  0.0f32, 1.0f32,
    -0.5f32, -0.5f32,  0.5f32,  0.0f32, 0.0f32,
    -0.5f32,  0.5f32,  0.5f32,  1.0f32, 0.0f32,

     0.5f32,  0.5f32,  0.5f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32, -0.5f32,  1.0f32, 1.0f32,
     0.5f32, -0.5f32, -0.5f32,  0.0f32, 1.0f32,
     0.5f32, -0.5f32, -0.5f32,  0.0f32, 1.0f32,
     0.5f32, -0.5f32,  0.5f32,  0.0f32, 0.0f32,
     0.5f32,  0.5f32,  0.5f32,  1.0f32, 0.0f32,

    -0.5f32, -0.5f32, -0.5f32,  0.0f32, 1.0f32,
     0.5f32, -0.5f32, -0.5f32,  1.0f32, 1.0f32,
     0.5f32, -0.5f32,  0.5f32,  1.0f32, 0.0f32,
     0.5f32, -0.5f32,  0.5f32,  1.0f32, 0.0f32,
    -0.5f32, -0.5f32,  0.5f32,  0.0f32, 0.0f32,
    -0.5f32, -0.5f32, -0.5f32,  0.0f32, 1.0f32,

    -0.5f32,  0.5f32, -0.5f32,  0.0f32, 1.0f32,
     0.5f32,  0.5f32, -0.5f32,  1.0f32, 1.0f32,
     0.5f32,  0.5f32,  0.5f32,  1.0f32, 0.0f32,
     0.5f32,  0.5f32,  0.5f32,  1.0f32, 0.0f32,
    -0.5f32,  0.5f32,  0.5f32,  0.0f32, 0.0f32,
    -0.5f32,  0.5f32, -0.5f32,  0.0f32, 1.0f32
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
    ebo_id: gl::GLuint,
    texture: gl::GLuint,

    // model: glm::Mat4,
    // view: glm::Mat4,
    // projection: glm::Mat4,
}

impl Renderer {
    pub fn init(camera: Camera) -> (
        Self,
        Shader,
    ) {
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

        let mut renderer = Self {
            glfw: glfw,
            window: window,
            events: events,

            camera: camera,
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            texture: 0,
        };

        // let mut renderer = Self::new(camera);
        let shader = Shader::new(
            "/Users/morlovs/Projects/rust/engine/game/vshader.glsl",
            "/Users/morlovs/Projects/rust/engine/game/fshader.glsl",
        );

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            

            gl::GenVertexArrays(1,  &mut renderer.vao_id as *mut gl::GLuint);
            gl::GenBuffers(1, &mut  renderer.vbo_id as *mut gl::GLuint);
            // gl::GenBuffers(1, &mut renderer.ebo_id as *mut gl::GLuint);




            gl::BindVertexArray(renderer.vao_id);

            
            gl::BindBuffer(gl::ARRAY_BUFFER, renderer.vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES).try_into().unwrap(),
                &VERTICES as *const _ as *const c_void,
                gl::STATIC_DRAW,
            );

            
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, renderer.ebo_id);
            // gl::BufferData(
            //     gl::ELEMENT_ARRAY_BUFFER,
            //     size_of_val(&INDICES).try_into().unwrap(),
            //     &INDICES as *const _ as *const c_void,
            //     gl::STATIC_DRAW,
            // );
            



            let a_pos: u32 = shader.get_attrib_location("aPos");
            let a_tex_coord: u32 = shader.get_attrib_location("aTexCoord");


            gl::EnableVertexAttribArray(a_pos);
            gl::VertexAttribPointer(
                a_pos,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * size_of::<gl::GLfloat>() as i32,
                std::ptr::null::<c_void>(),
            );

            gl::EnableVertexAttribArray(a_tex_coord);
            gl::VertexAttribPointer(
                a_tex_coord,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * size_of::<gl::GLfloat>() as i32,
                std::ptr::null::<c_void>().offset(3 * size_of::<gl::GLfloat>() as isize),
            );

            renderer.load_textures();


            println!("vao: {:?}, vbo: {:?}, vpoint: {:?}", renderer.vao_id, renderer.vbo_id, a_pos);
        }

        // let shader = renderer.init();

        (renderer, shader)
    }

    pub fn draw(&self, shader: &Shader) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let mut model = glm::Mat4::identity();
            model = glm::rotate(&model, self.glfw.get_time() as f32, &glm::vec3(1.0, 0.0, 0.0));

            let view = self.camera.get_view_matrix();

            let projection = glm::perspective(45.0f32.to_radians(), 800.0f32 / 600.0f32, 0.1f32, 100.0f32);

            shader.use_program();

            let model_loc = shader.get_uniform_location("model").try_into().unwrap();
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, glm::value_ptr(&model).as_ptr());

            let view_loc = shader.get_uniform_location("view").try_into().unwrap();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, glm::value_ptr(&view).as_ptr());

            let projection_loc = shader.get_uniform_location("projection").try_into().unwrap();
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, glm::value_ptr(&projection).as_ptr());



            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::BindVertexArray(self.vao_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null::<c_void>());
        }
    }

    fn load_textures(&mut self) {    
        unsafe {
            gl::GenTextures(1, &mut self.texture as *mut gl::GLuint);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

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
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );

            let img = image::open("/Users/morlovs/Projects/rust/engine/game/container.jpg")
                .unwrap()
                .flipv()
                .into_rgba8();
        
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
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
