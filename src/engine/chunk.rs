use crate::primitives::InstanceAttr;
use glium::vertex::PerInstance;
use glium::{Display, VertexBuffer};

use super::Block;

pub const CHUNK_SIZE: usize = 16 * 16 * 256;
pub struct Chunk {
    blocks: Vec<Block>,
    vbo: Option<VertexBuffer<InstanceAttr>>
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut c= Chunk { blocks: Vec::new(), vbo: None };
        c.add_plane();
        c
    }

    pub fn add_plane(&mut self) {
      for x in 0..16 {
        for z in 0..16 {
          self.blocks.push(Block {
            position: (x as f32, 0., z as f32)
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
