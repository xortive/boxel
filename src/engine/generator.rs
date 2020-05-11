use super::block::BlockType;
use super::chunk::Chunk;
use super::types::*;
use crate::config::*;

use glm::{vec2, vec3};
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
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block_type = match (x + z) % 4 {
                    std::u32::MIN..=0 => BlockType::DIRT,
                    1 => BlockType::SAND,
                    2 => BlockType::STONE,
                    3..=std::u32::MAX => BlockType::GRASS,
                };
                chunk.add_block(vec3(x, 0, z), block_type);
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

    fn get_block(&self, y: u32) -> BlockType {
        match (y as i32) - (HEIGHT_OFFSET as i32) {
            std::i32::MIN..=0 => BlockType::SAND,
            1..=12 => BlockType::GRASS,
            13..=std::i32::MAX => BlockType::STONE
        }
    }
}

impl WorldGenerator for PerlinGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk {
        let mut chunk = Chunk::new(coordinate);
        let chunk_world: glm::TVec2<f64> = glm::convert(chunk.world_origin().xz());
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block_world: [f64; 2] = ((chunk_world + vec2(x as f64, z as f64)) * 0.015).into();
                let y = self.perlin.get(block_world);
                let y = y.powi(4) * HEIGHT_OFFSET as f64;
                let y = (y + HEIGHT_OFFSET as f64) as u32;
                //println!("u32 y was {}", y);

                // println!("pos: {:#?} y: {:?}", block_world, y);
                chunk.add_block(vec3(x, y, z), self.get_block(y));
                
                let mut i = 1;
                for _ in y..0 {
                    //println!("Adding water {} {}", i, y);
                    chunk.add_block(vec3(x, y + i, z), BlockType::WATER);
                    i += 1;
                }

                i = 1;
                for _ in 0..y {
                    //println!("Adding stone {} {}", i, y);
                    chunk.add_block(vec3(x, y - i, z), BlockType::STONE);
                    i += 1;
                }
            }
        }
        chunk
    }
}
