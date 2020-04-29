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
        c.add_block();
        c
    }

    pub fn add_block(&mut self) {
      self.blocks.push(Block {
        position: (0. , 0. , 0.)
      });
    }

    //loads instance if they don't exist
    pub fn per_instance(&mut self, display: &Display) -> PerInstance {
        if self.vbo.is_none() {
          let instances: Vec<InstanceAttr> =
          self.blocks.clone().into_iter().map(|b| b.into()).collect();

          let vbo = VertexBuffer::new(display, &instances).expect("to create vb");

          self.vbo = Some(vbo);
        }

        self.vbo.as_ref().unwrap().per_instance().unwrap()
    }
}
