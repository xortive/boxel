use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, ChunkCoordinate};
use crate::config::HEIGHT_OFFSET;

use glm::{vec2};
use noise::{Perlin, NoiseFn, Seedable};

pub trait WorldGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk;
}

pub struct PlanarGenerator {}

impl PlanarGenerator {
    pub fn new() -> PlanarGenerator {
        PlanarGenerator {}
    }
}

impl WorldGenerator for PlanarGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk {
        let mut chunk = Chunk::new(coordinate);
        for x in 0..16 {
            for z in 0..16 {
                let block_type = match (x + z) % 4 {
                    std::i32::MIN..=0 => BlockType::DIRT,
                    1 => BlockType::SAND,
                    2 => BlockType::STONE,
                    3..=std::i32::MAX => BlockType::GRASS,
                };
                // chunk.add_block(glm::vec3(x as f32, 0., z as f32), block_type);
            }
        }
        chunk
    }
}

pub struct PerlinGenerator {
    perlin: Perlin
}

impl PerlinGenerator {
    pub fn new() -> PerlinGenerator {
        let perlin = Perlin::new();
        perlin.set_seed(1234);
        PerlinGenerator {
            perlin
        }
    }

    fn get_block(&self, y: i32) -> BlockType {
        match y {
            std::i32::MIN..=0 => BlockType::SAND,
            1..=12 => BlockType::GRASS,
            13..=std::i32::MAX => BlockType::STONE
        }
    }
}

impl WorldGenerator for PerlinGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk {
        let mut chunk = Chunk::new(coordinate);
        let chunk_world: glm::TVec2<f64> = vec2(chunk.world_origin()[0] as f64, chunk.world_origin()[2] as f64);
        for x in 0..16 {
            for z in 0..16 {
                let block_world: [f64; 2] = ((chunk_world + vec2(x as f64, z as f64)) * 0.015).into();
                let y: i32 = (self.perlin.get(block_world).powi(3) * 16. as f64) as i32;
                // println!("pos: {:#?} y: {:?}", block_world, y);
                chunk.add_block([x, y, z].into(), self.get_block(y));
                
                let mut i = 1;
                for _ in y..-1 {
                    // println!("Adding water");
                    chunk.add_block([x, (y + i), z].into(), BlockType::WATER);
                    i += 1;
                }

                let min_y = -HEIGHT_OFFSET;
                i = 1;
                for _ in min_y..y - 1 {
                    chunk.add_block([x, (y - i), z].into(), BlockType::STONE);
                    i += 1;
                }
            }
        }
        chunk
    }
}
