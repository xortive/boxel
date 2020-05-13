use crate::primitives::Vertex;
use crate::camera::CameraState;
use genmesh::{self, generators::Cube, MapToVertices, Triangulate, Vertices};
use glium::glutin::event::VirtualKeyCode;
use glium::vertex::VertexBuffer;
use glium::{program, uniform};
use glium::{Display, Surface};

use glm::{vec3, vec4};

mod block;
mod chunk;
mod crosshair;
mod march;
pub mod generator;
mod world;
use world::World;
use crosshair::Crosshair;

use std::time::Duration;

pub struct Engine {
    pub camera: CameraState,
    pub display: Box<Display>,
    cube: VertexBuffer<Vertex>,
    program: glium::Program,
    world: World,
    grab: bool,
    crosshair: Crosshair,
    crosshair_program: glium::Program,
    text_system: glium_text::TextSystem,
    font: glium_text::FontTexture,
}

impl Engine {
    pub fn new(display: Display, seed: u32) -> Engine {
        // the program
        let program = program!(&display,
            140 => {vertex: include_str!("./../shaders/vertex.glsl"), fragment: include_str!("./../shaders/fragment.glsl")},
        )
        .unwrap();

        let crosshair_program = program!(&display,
            140 => {vertex: include_str!("./../shaders/crosshair_vertex.glsl"), fragment: include_str!("./../shaders/crosshair_fragment.glsl")},
        )
        .unwrap();

        let cube = {
            let cube_vertices: Vec<Vertex> = Cube::new()
                .vertex(|v| {
                    let pos = vec3(v.pos.x, v.pos.y, v.pos.z).scale(0.5);
                    Vertex::new(pos.into(), v.normal.into())
                })
                .triangulate()
                .vertices()
                .collect();
            // for test in cube_vertices.iter() {
            //     println!("CUBE [{}, {}, {}]", test.position[0], test.position[1], test.position[2]);
            // }
            VertexBuffer::new(&display, cube_vertices.as_slice()).unwrap()
        };

        let camera = CameraState::new();

        let world = World::new(seed);

        let crosshair = Crosshair::new(&display);

        let text_system = glium_text::TextSystem::new(&display);
        let font = glium_text::FontTexture::new(&display, &include_bytes!("../../fonts/minecraft.ttf")[..], 12).unwrap();

        let display = Box::new(display);

        Engine {
            camera,
            cube,
            display,
            program,
            world,
            grab: true,
            crosshair,
            crosshair_program,
            text_system,
            font,
        }
    }

    pub fn render(&mut self, fps: f32) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.529, 0.808, 0.980, 1.0), 1.0);

        self.display
            .gl_window()
            .window()
            .set_cursor_grab(self.grab)
            .unwrap();
        self.display
            .gl_window()
            .window()
            .set_cursor_visible(!self.grab);

        self.camera.update();

        // building the uniforms
        let uniforms = uniform! {
          persp_matrix: *(self.camera.get_perspective().as_ref()),
          view_matrix: *(self.camera.get_view().as_ref()),
        };

        // draw coordinates to screen
        let (w, h) = self.display.get_framebuffer_dimensions();

        let mut i = 0;
        for coord in ["X", "Y", "Z"].iter() {
            let coord_text = glium_text::TextDisplay::new(&self.text_system,
                &self.font,
                &format!("{}: {:.2}", coord, self.camera.get_position()[i]));
                            
            const TEXT_SIZE: f32 = 0.04;
            let matrix: [[f32; 4]; 4] = [
                [TEXT_SIZE, 0.0, 0.0, 0.0],
                [0.0, TEXT_SIZE * (w as f32) / (h as f32), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-0.95, 0.9 - ((i as f32) * 0.1), 0.0, 1.0f32]
            ];
            i += 1;
            glium_text::draw(&coord_text, &self.text_system, &mut target, matrix, (1.0, 1.0, 1.0, 1.0));
        }
                        

        // let fps_text = glium_text::TextDisplay::new(&self.text_system,
        //     &self.font,
        //     &format!("FPS: {:.0}", fps));

        // matrix = [
        //     [TEXT_SIZE, 0.0, 0.0, 0.0],
        //     [0.0, TEXT_SIZE * (w as f32) / (h as f32), 0.0, 0.0],
        //     [0.0, 0.0, 1.0, 0.0],
        //     [-0.95, 0.8, 0.0, 1.0f32]
        // ];

        // glium_text::draw(&fps_text, &self.text_system, &mut target, matrix, (1.0, 1.0, 1.0, 1.0));

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::draw_parameters::Blend::alpha_blending(),
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        const INDICES: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let camera_pos = self.camera.get_position();

        self.world.update_chunks(camera_pos, &self.display);

        for chunk in self.world.rendered_chunks(camera_pos).iter() {
            if let Some(instances) = chunk.per_instance() {
                target
                    .draw(
                        (&self.cube, instances),
                        &INDICES,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

        }

        target.draw(&self.crosshair.vbo, &glium::index::NoIndices(glium::index::PrimitiveType::LinesList), &self.crosshair_program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        
        target.finish().unwrap();
    }

    pub fn process_click(&mut self) {
        println!("Process click");
        let eye = self.camera.get_position();
        let view = self.camera.get_view();
        let proj = self.camera.get_perspective();

        let near = glm::unproject(&vec3(1024./2.,768./2.,-1.), &view, &proj, vec4(0.,0.,1024.,768.));
        let far = glm::unproject(&vec3(1024./2.,768./2.,1.), &view, &proj, vec4(0.,0.,1024.,768.));
        let ray = glm::normalize(&(far-near));

        // let ray = Ray::new([eye[0], eye[1], eye[2]].into(), [ray[0], ray[1], ray[2]].into());
        // println!("Ray origin: {} dir: {} far {} eye {}", ray.origin, ray.dir, far, eye);
        self.world.intersect(&eye, &ray);
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
