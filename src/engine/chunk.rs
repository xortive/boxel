use crate::config::*;
use super::block::{Block, BlockProperties, BlockType};
use super::types::*;
use crate::primitives::InstanceAttr;
use glium::vertex::PerInstance;
use glium::{Display, VertexBuffer};
use glm::{vec3, UVec3, IVec3};

pub struct Chunk {
    coordinates: ChunkCoordinate, //in chunk space, so (0, 0) is the chunk from worldspace (0,y,0) to (16,y,16);
    //blocks[x][z][y]
    blocks: [[[Option<BlockProperties>; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    vbo: Option<VertexBuffer<InstanceAttr>>,
}

impl Chunk {
    pub fn new(coordinates: ChunkCoordinate) -> Chunk {
        let c = Chunk {
            coordinates,
            blocks: [[[None; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
            vbo: None,
        };
        // c.add_plane();
        c
    }

    pub fn add_block(&mut self, coordinate: BlockCoordinate, block_type: BlockType) {
        //println!("coords {:?}", coordinate);
        self.blocks[coordinate.x as usize][coordinate.z as usize][coordinate.y as usize] = Some(BlockProperties { block_type });
    }

    pub fn world_origin(&self) -> WorldCoordinate {
        let world = self.coordinates * CHUNK_SIZE as i32;
        vec3(world[0], 0, world[1])
    }

    pub fn update_vbo(&mut self, display: &Display) {
        let mut instances: Vec<InstanceAttr> = Vec::new();
        for x in 0..CHUNK_SIZE as usize {
            for z in 0..CHUNK_SIZE as usize {
                for y in 0..CHUNK_HEIGHT as usize {
                    if let Some(props) = self.blocks[x][z][y] {
                        let pos = self.world_origin() + vec3(x as i32, y as i32, z as i32);
                        instances.push(Block::new(pos, props).into());
                    }
                }
            }
        }

        println!("{} instances", instances.len());

        let vbo = VertexBuffer::new(display, &instances).expect("to create vb");

        self.vbo = Some(vbo);
    }

    //loads instance if they don't exist
    pub fn per_instance(&self) -> PerInstance {
        self.vbo.as_ref().unwrap().per_instance().unwrap()
    }
}