use crate::support::camera::CameraState;
use crate::primitives::{Vertex, InstanceAttr};
use genmesh;
use genmesh::{Vertices, MapToVertices, Triangulate};
use genmesh::{Quad, generators::Cube};
use glium::glutin::event::VirtualKeyCode;
use glium::vertex::VertexBuffer;
use glium::{program, uniform, implement_vertex};
use glium::{Display, Surface};

pub struct Engine {
    pub camera: CameraState,
    cube: VertexBuffer<Vertex>,
    cubes: VertexBuffer<InstanceAttr>,
    program: glium::Program,
}

impl Engine {
    pub fn new(display: &Display) -> Engine {
        // the program
        let program = program!(display,
            140 => {vertex: include_str!("./shaders/vertex.glsl"), fragment: include_str!("./shaders/fragment.glsl")},
        )
        .unwrap();

        let cube_vertices: Vec<Vertex> = Cube::new()
          .vertex(|v| Vertex::new(v.pos.into(), v.normal.into()))
          .triangulate()
          .vertices()
          .collect();

        let cube = VertexBuffer::new(display, cube_vertices.as_slice()).unwrap();

        let cubes = {
          let data = [
              InstanceAttr {
                  world_position: (0.0, 0.0, 0.0),
              }
          ];
  
          VertexBuffer::dynamic(display, &data).unwrap()
        };

        let mut camera = CameraState::new();

        camera.set_position((10.,10.,0.));
        camera.set_direction((-0.5,-0.5,0.));

        Engine {
            camera,
            cube,
            cubes,
            program,
        }
    }

    pub fn render(&mut self, target: &mut glium::Frame) {
        // building the uniforms
        self.camera.update();
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
        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);


        target.draw((&self.cube, self.cubes.per_instance().unwrap()),
                    &INDICES, 
                    &self.program, 
                    &uniforms,
                    &params).unwrap();
    }

    pub fn process_keyboard(&mut self, pressed: bool, key: VirtualKeyCode) {
        self.camera.process_input(pressed, key);
    }
}
