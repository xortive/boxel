extern crate nalgebra_glm as glm;
use glium::glutin;
use glium::glutin::event::VirtualKeyCode;
use glm::{Vec3, Mat4};
//TODO clean
pub struct CameraState {
    position: (f32, f32, f32),
    direction: (f32, f32, f32),
    eye: Vec3,
    up: Vec3,
    look: Vec3,
    center: Vec3,
    keys: Vec<VirtualKeyCode>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    const CAMERA_DISTANCE: f32 = 10.0;
    const ZOOM_SPEED: f32 = 0.1;
    const PAN_SPEED: f32 = 0.05;
}

impl CameraState {
    pub fn new() -> CameraState {
        let eye: Vec3 = glm::vec3(CameraState::CAMERA_DISTANCE, CameraState::CAMERA_DISTANCE, 0.0);
        let up: Vec3 = glm::vec3(0.0, 1.0, 0.0);
        let look: Vec3 = glm::vec3(-0.5, -0.5, 0.0);
        let center: Vec3 = eye - CameraState::CAMERA_DISTANCE * look;
 
        CameraState {
            position: (0.1, 0.1, 1.0),
            direction: (0.0, 0.0, -1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            eye: eye,
            up: up, 
            look: look, 
            center: center,
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
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);

        if self.moving_up {
            self.eye += CameraState::PAN_SPEED * self.up; 
        }

        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }

        if self.moving_down {
            self.eye -= CameraState::PAN_SPEED * self.up; 
        }

        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }

        if self.moving_forward {
            self.eye += CameraState::ZOOM_SPEED * self.look;
        }

        if self.moving_backward {
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

        } else {

        }

        match key {
            glutin::event::VirtualKeyCode::Up => self.moving_up = pressed,
            glutin::event::VirtualKeyCode::Down => self.moving_down = pressed,
            glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
        };
    }
}