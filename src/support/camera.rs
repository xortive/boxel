extern crate nalgebra_glm as glm;
use glium::glutin::event::VirtualKeyCode;
use glm::{Vec3, Mat4};

pub struct CameraState {
    eye: Vec3,
    up: Vec3,
    look: Vec3,
    center: Vec3,
    tangent: Vec3,
    keys: Vec<VirtualKeyCode>,
}

impl CameraState {
    const CAMERA_DISTANCE: f32 = 10.0;
    const ZOOM_SPEED: f32 = 0.1;
    const PAN_SPEED: f32 = 0.1;
}

impl CameraState {
    pub fn new() -> CameraState {
        let eye: Vec3 = glm::vec3(CameraState::CAMERA_DISTANCE, CameraState::CAMERA_DISTANCE, 0.0);
        let up: Vec3 = glm::vec3(0.0, 1.0, 0.0);
        let look: Vec3 = glm::vec3(-0.5, -0.5, 0.0);
        let center: Vec3 = eye - CameraState::CAMERA_DISTANCE * look;
        let tangent: Vec3 = glm::cross(&look, &up);
 
        CameraState {
            eye: eye,
            up: up, 
            look: look, 
            center: center,
            tangent: tangent,
            keys: Vec::new(),
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
        return glm::look_at(&self.eye, &self.center, &self.up);
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
}