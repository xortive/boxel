use glium::glutin::event::VirtualKeyCode;
use nalgebra_glm as glm;
use glm::{Vec2, Vec3, Vec4, vec2, vec3, vec4, Mat4};

use std::time::Duration;

#[derive(Debug)]
pub struct CameraState {
    eye: Vec3,
    look: Vec3,
    up: Vec3,
    keys: Vec<VirtualKeyCode>,
}

const CAMERA_DISTANCE: f32 = 10.0;
const ZOOM_SPEED: f32 = 0.1;
const PAN_SPEED: f32 = 0.1;
const ROTATION_SPEED: f32 = 0.05;

impl CameraState {
    pub fn new() -> CameraState {
        let eye: Vec3 = vec3(8.0, 4.0, 8.0);
        let look = vec3(0.,0.,1.);
        let up = vec3(0.,1.,0.);
 
        CameraState {
            eye,
            look,
            up,
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
        glm::look_at(&self.eye, &(self.eye-self.look), &self.up)
    }

    pub fn update(&mut self) {
        let tangent = glm::cross(&self.look, &self.up);

        if self.keys.contains(&VirtualKeyCode::W) {
            self.eye += ZOOM_SPEED * self.look;
        }

        if self.keys.contains(&VirtualKeyCode::Up) {
            self.eye += PAN_SPEED * self.up; 
        }

        if self.keys.contains(&VirtualKeyCode::A) {
            self.eye += PAN_SPEED * tangent;
        }

        if self.keys.contains(&VirtualKeyCode::S) {
            self.eye -= ZOOM_SPEED * self.look;
        }

        if self.keys.contains(&VirtualKeyCode::Down) {
            self.eye -= PAN_SPEED * self.up; 
        }

        if self.keys.contains(&VirtualKeyCode::D) {
            self.eye -= PAN_SPEED * tangent;
        }
    }

    pub fn process_input(&mut self, pressed: bool, key: VirtualKeyCode, dt: Duration) {
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

    pub fn process_cursor(&mut self, delta: (f64, f64), dt: Duration) {
        let angle = ROTATION_SPEED * dt.as_secs_f32();

        let rotate_y = glm::quat_angle_axis(angle * delta.1 as f32, &-glm::cross(&self.look, &glm::vec3(0.,1.,0.)));
        let rotate_x = glm::quat_angle_axis(angle * delta.0 as f32, &glm::vec3(0.,1.,0.));

        let rotate = rotate_x * rotate_y;

        self.look = glm::quat_rotate_vec3(&rotate, &self.look);
        self.up = glm::quat_rotate_vec3(&rotate, &self.up);
    }
}