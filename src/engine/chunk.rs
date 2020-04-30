use crate::primitives::InstanceAttr;
use glium::vertex::PerInstance;
use glium::{Display, VertexBuffer};
use glm::{vec2, vec3, Vec2, Vec3};

use super::Block;

pub const CHUNK_SIZE: usize = 16 * 16 * 256;
pub struct Chunk {
    coordinates: Vec2, //in chunk space, so (0, 0) is the chunk from worldspace (0,y,0) to (16,y,16);
    blocks: Vec<Block>,
    vbo: Option<VertexBuffer<InstanceAttr>>,
}

impl Chunk {
    pub fn new(coordinates: Vec2) -> Chunk {
        let mut c = Chunk {
            coordinates,
            blocks: Vec::new(),
            vbo: None,
        };
        c.add_plane();
        c
    }

    pub fn world_origin(&self) -> Vec3 {
        let world = self.coordinates * 16.;
        vec3(world.x, 0., world.y)
    }

    pub fn add_plane(&mut self) {
        let origin = self.world_origin();
        for x in 0..16 {
            for z in 0..16 {
                let world_x = x as f32 + origin.x;
                let world_z = z as f32 + origin.z;
                self.blocks.push(Block {
                    position: (world_x as f32, 0., world_z as f32),
                })
            }
        }
    }

    //loads instance if they don't exist
    pub fn per_instance(&mut self, display: &Display) -> PerInstance {
        if self.vbo.is_none() {
            let instances: Vec<InstanceAttr> =
                self.blocks.clone().into_iter().map(|b| b.into()).collect();

            println!("{} instances", instances.len());

            let vbo = VertexBuffer::new(display, &instances).expect("to create vb");

            self.vbo = Some(vbo);
        }

        self.vbo.as_ref().unwrap().per_instance().unwrap()
    }
}
