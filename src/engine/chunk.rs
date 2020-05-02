use crate::engine::block::{Block, BlockType};
use crate::primitives::InstanceAttr;
use glium::vertex::PerInstance;
use glium::{Display, VertexBuffer};
use glm::{vec2, vec3, Vec2, Vec3};
use nalgebra::Point2;

pub const CHUNK_SIZE: usize = 16;

pub type ChunkCoordinate = Point2<i32>; // chunk space
pub type BlockCoordinate = Vec3; // block space in a chunk (CHUNK_LENGTH x CHUNK_WIDTH x CHUNK_HEIGHT)

pub struct Chunk {
    coordinates: ChunkCoordinate, //in chunk space, so (0, 0) is the chunk from worldspace (0,y,0) to (16,y,16);
    blocks: Vec<Block>,
    vbo: Option<VertexBuffer<InstanceAttr>>,
}

impl Chunk {
    pub fn new(coordinates: ChunkCoordinate) -> Chunk {
        let mut c = Chunk {
            coordinates,
            blocks: Vec::new(),
            vbo: None,
        };
        // c.add_plane();
        c
    }

    pub fn add_block(&mut self, coordinate: BlockCoordinate, block_type: BlockType) {
        // first, convert to world space
        let origin = self.world_origin();

        let world_space = origin + coordinate;
        self.blocks.push(Block {
            position: (world_space[0], world_space[1], world_space[2]),
            block_type: block_type,
        });
    }

    pub fn world_origin(&self) -> Vec3 {
        let world = self.coordinates * 16;
        vec3(world[0] as f32, 0., world[1] as f32)
    }

    // pub fn add_plane(&mut self) {
    //     let origin = self.world_origin();
    //     for x in 0..16 {
    //         for z in 0..16 {
    //             let world_x = x as f32 + origin.x;
    //             let world_z = z as f32 + origin.z;
    //             let color = (x + z) % 4;

    //             println!("{} {} {}", x, z, color);

    //             self.blocks.push(Block {
    //                 position: (world_x as f32, 0., world_z as f32),
    //                 color
    //             })
    //         }
    //     }
    // }

    pub fn update_vbo(&mut self, display: &Display) {
        let instances: Vec<InstanceAttr> =
            self.blocks.clone().into_iter().map(|b| b.into()).collect();

        println!("{} instances", instances.len());

        let vbo = VertexBuffer::new(display, &instances).expect("to create vb");

        self.vbo = Some(vbo);
    }

    //loads instance if they don't exist
    pub fn per_instance(&self) -> PerInstance {
        self.vbo.as_ref().unwrap().per_instance().unwrap()
    }
}
