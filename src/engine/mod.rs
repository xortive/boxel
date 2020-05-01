use crate::primitives::{InstanceAttr, Vertex};
use crate::support::camera::CameraState;
use genmesh::{self, generators::Cube, MapToVertices, Triangulate, Vertices};
use glium::glutin::event::VirtualKeyCode;
use glium::vertex::VertexBuffer;
use glium::{program, uniform};
use glium::{Display, Surface};

use glm::{
  vec2, Vec2, vec3
};

mod block;
mod chunk;
mod world;
pub mod generator;
use world::World;

use std::time::Duration;

pub struct Engine {
    pub camera: CameraState,
    pub display: Display,
    cube: VertexBuffer<Vertex>,
    program: glium::Program,
    world: World,
    grab: bool,
}

impl Engine {
    pub fn new(display: Display) -> Engine {
        // the program
        let program = program!(&display,
            140 => {vertex: include_str!("./../shaders/vertex.glsl"), fragment: include_str!("./../shaders/fragment.glsl")},
        )
        .unwrap();

        let cube = {
            let cube_vertices: Vec<Vertex> = Cube::new()
                .vertex(|v| {
                  let pos = vec3(v.pos.x, v.pos.y, v.pos.z).scale(0.5);
                  Vertex::new(pos.into(), v.normal.into())}
                )
                .triangulate()
                .vertices()
                .collect();

            VertexBuffer::new(&display, cube_vertices.as_slice()).unwrap()
        };

        let camera = CameraState::new();

        let world = World::new();

        Engine {
            camera,
            cube,
            display,
            program,
            world,
            grab: true
        }
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.529, 0.808, 0.980, 1.0), 1.0);

        self.display.gl_window().window().set_cursor_grab(self.grab).unwrap();
        self.display.gl_window().window().set_cursor_visible(!self.grab);

        self.camera.update();

        // building the uniforms
        let uniforms = uniform! {
          persp_matrix: *(self.camera.get_perspective().as_ref()),
          view_matrix: *(self.camera.get_view().as_ref()),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        const INDICES: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let camera_pos = self.camera.get_position();

        self.world.update_chunks(camera_pos, &self.display);

        for chunk in self.world.rendered_chunks(camera_pos).iter() {
            let instance = chunk.per_instance();

            target
                .draw(
                    (&self.cube, instance),
                    &INDICES,
                    &self.program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
        target.finish().unwrap();
    }

    pub fn process_keyboard(&mut self, pressed: bool, key: VirtualKeyCode, dt: Duration) {
        if key == VirtualKeyCode::Escape && pressed {
          self.grab = !self.grab;
        } else {
          self.camera.process_input(pressed, key, dt);
        }
    }

    pub fn process_cursor(&mut self, position: (f64, f64), dt: Duration) {
      if self.grab {
        self.camera.process_cursor(position, dt);
      }
    }
  }
