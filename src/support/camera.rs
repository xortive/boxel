extern crate nalgebra_glm as glm;
use glium::glutin::event::VirtualKeyCode;
use glm::{Vec3, Mat4, Mat3};

#[derive(Debug)]
pub struct CameraState {
    eye: Vec3,
    up: Vec3,
    look: Vec3,
    tangent: Vec3,
    orientation: Mat3,
    keys: Vec<VirtualKeyCode>,
    last_position: (f64, f64),
}

impl CameraState {
    const CAMERA_DISTANCE: f32 = 10.0;
    const ZOOM_SPEED: f32 = 0.1;
    const PAN_SPEED: f32 = 0.1;
    const ROTATION_SPEED: f32 = 0.1;
}

impl CameraState {
    pub fn new() -> CameraState {
        let eye: Vec3 = glm::vec3(CameraState::CAMERA_DISTANCE, CameraState::CAMERA_DISTANCE, 0.0);
        let up: Vec3 = glm::vec3(0.0, 1.0, 0.0);
        let look: Vec3 = glm::vec3(-0.5, -0.5, 0.0);
        let tangent: Vec3 = glm::cross(&look, &up);
        let orientation: Mat3 = glm::mat3(tangent[0], tangent[1], tangent[2], up[0], up[1], up[2], look[0], look[1], look[2]);
 
        CameraState {
            eye: eye,
            up: up, 
            look: look, 
            tangent: tangent,
            orientation: orientation,
            keys: Vec::new(),
            last_position: (-1.0, -1.0),
        }
    }

    pub fn get_perspective(&self) -> Mat4 {
        let aspect = 1024.0 / 768.0;
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 1.0;
        
        return glm::perspective_lh(aspect, fov, znear, zfar);
    }

    pub fn get_view(&self) -> Mat4 {
        let center = self.eye - CameraState::CAMERA_DISTANCE * self.look;
        return glm::look_at(&self.eye, &center, &self.up);
    }

    pub fn update(&mut self) {
        if self.keys.contains(&VirtualKeyCode::Up) {
            self.eye += CameraState::PAN_SPEED * self.up; 
        }

        if self.keys.contains(&VirtualKeyCode::A) {
            self.eye += CameraState::PAN_SPEED * self.tangent;
        }

        if self.keys.contains(&VirtualKeyCode::Down) {
            self.eye -= CameraState::PAN_SPEED * self.up; 
        }

        if self.keys.contains(&VirtualKeyCode::D) {
            self.eye -= CameraState::PAN_SPEED * self.tangent;
        }

        if self.keys.contains(&VirtualKeyCode::W) {
            self.eye += CameraState::ZOOM_SPEED * self.look;
        }

        if self.keys.contains(&VirtualKeyCode::S) {
            self.eye -= CameraState::ZOOM_SPEED * self.look;
        }
    }

    pub fn process_input(&mut self, pressed: bool, key: VirtualKeyCode) {
        println!(
            "{} key: {:#?}!",
            if pressed { "Pressed" } else { "Released" },
            key
        );

        if pressed {
            self.keys.push(key);
        } else {
            self.keys.retain(|&x| { x != key });
        }
    }

    pub fn process_cursor(&mut self, position: (f64, f64)) {
        if self.last_position.0 == -1.0 {
            self.last_position.0 = position.0;
            self.last_position.1 = position.1; 
            return;
        }

        let delta_x = position.0 - self.last_position.0;
        let delta_y = position.1 - self.last_position.1;
        if (delta_x * delta_x + delta_y * delta_y).sqrt() < 1e-15 {
            print!("returning");
            return; 
        }

        println!("X: {} Y: {}", delta_x, delta_y);
        let mouse_dir = glm::normalize(&glm::vec3(delta_x, delta_y, 0.0));
        let mut axis = self.orientation * glm::make_vec3(&[mouse_dir[0] as f32, mouse_dir[1] as f32, 0.0]);
        axis = glm::normalize(&axis);

        self.orientation = glm::mat4_to_mat3(&glm::rotate(&glm::mat3_to_mat4(&self.orientation), CameraState::ROTATION_SPEED, &axis));
        self.tangent = glm::column(&self.orientation, 0);
        self.up = glm::column(&self.orientation, 1);
        self.look = glm::column(&self.orientation, 2);

        self.last_position.0 = position.0;
        self.last_position.1 = position.1;
    }
}