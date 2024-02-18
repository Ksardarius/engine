const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT
}

#[derive(Debug)]
pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    // euler angles
    yaw: f32,
    pitch: f32,

    //camera options
    movement_speed: f32,
    mouse_sensivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: Option<glm::Vec3>, up: Option<glm::Vec3>, yaw: Option<f32>, pitch: Option<f32>) -> Self {
        let mut cam = Camera {
            position: position.unwrap_or(glm::vec3(0.0, 0.0, 0.0)),
            world_up: up.unwrap_or(glm::vec3(0.0, 1.0, 0.0)),
            yaw: yaw.unwrap_or(YAW),
            pitch: pitch.unwrap_or(PITCH),
            front: glm::vec3(0.0, 0.0, -1.0),
            movement_speed: SPEED,
            mouse_sensivity: SENSITIVITY,
            zoom: ZOOM,

            up: glm::Vec3::default(),
            right: glm::Vec3::default()
        };

        cam.update_camera_vectors();

        cam
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, dt: f32) {
        let velocity = self.movement_speed * dt;
        match direction {
            CameraMovement::FORWARD => self.position += self.front * velocity,
            CameraMovement::BACKWARD => self.position -= self.front * velocity,
            CameraMovement::LEFT => self.position -= self.right * velocity,
            CameraMovement::RIGHT => self.position += self.right * velocity,
        }
    }

    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32, constraint_pitch: bool) {
        xoffset *= self.mouse_sensivity;
        yoffset *= self.mouse_sensivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        // make sure that when pitch is out of bounds, screen doesn't get flipped
        if constraint_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }

            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        // update Front, Right and Up Vectors using the updated Euler angles
        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        self.zoom -= yoffset;

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }

        if self.zoom > 45.0 {
            self.zoom = 45.0
        }
    }

    fn update_camera_vectors(&mut self) {
        let mut front = glm::Vec3::default();
        front.x = f32::cos(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
        front.y = f32::sin(f32::to_radians(self.pitch));
        front.z = f32::sin(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));

        self.front = glm::normalize(&front);

        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}
