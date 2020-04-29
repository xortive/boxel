use crate::primitives::{InstanceAttr, Vertex};
use crate::support::camera::CameraState;
use genmesh::{self, generators::Cube, MapToVertices, Triangulate, Vertices};
use glium::glutin::event::VirtualKeyCode;
use glium::vertex::VertexBuffer;
use glium::{program, uniform};
use glium::{Display, Surface};

mod block;
mod chunk;

use block::Block;
use chunk::{Chunk, CHUNK_SIZE};

pub struct Engine {
    pub camera: CameraState,
    pub display: Display,
    cube: VertexBuffer<Vertex>,
    program: glium::Program,
    chunks: Vec<Chunk>,
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
                .vertex(|v| Vertex::new(v.pos.into(), v.normal.into()))
                .triangulate()
                .vertices()
                .collect();

            VertexBuffer::new(&display, cube_vertices.as_slice()).unwrap()
        };

        let mut camera = CameraState::new();

        camera.set_position((10., 10., 0.));
        camera.set_direction((-0.5, -0.5, 0.));

        let mut chunks = Vec::new();
        chunks.push(Chunk::new());

        Engine {
            camera,
            cube,
            chunks,
            display,
            program,
        }
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.529, 0.808, 0.980, 1.0), 1.0);

        // building the uniforms
        let uniforms = uniform! {
          persp_matrix: self.camera.get_perspective(),
          view_matrix: self.camera.get_view(),
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

        for chunk in self.chunks.iter_mut() {
            let instance = chunk.per_instance(&self.display);

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

    pub fn process_keyboard(&mut self, pressed: bool, key: VirtualKeyCode) {
        println!(
            "{} key: {:#?}!",
            if pressed { "Pressed" } else { "Released" },
            key
        );
    }
}
