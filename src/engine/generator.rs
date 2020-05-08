use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, ChunkCoordinate};
use crate::engine::chunk::CHUNK_SIZE;

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
                chunk.add_block(glm::vec3(x as f32, 0., z as f32), block_type);
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
}

impl WorldGenerator for PerlinGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk {
        let mut chunk = Chunk::new(coordinate);
        let chunk_world: glm::TVec2<f64> = glm::convert(chunk.world_origin().xz());
        for x in 0..16 {
            for z in 0..16 {
                let block_type = match (x + z) % 4 {
                    std::i32::MIN..=0 => BlockType::DIRT,
                    1 => BlockType::SAND,
                    2 => BlockType::STONE,
                    3..=std::i32::MAX => BlockType::GRASS,
                };
                let block_world: [f64; 2] = ((chunk_world + vec2(x as f64, z as f64)) * 0.015).into();
                let y: f64 = self.perlin.get(block_world).powf(5.15) * 16 as f64;
                println!("pos: {:#?} y: {:?}", block_world, y);
                chunk.add_block(glm::vec3(x as f32, y as f32, z as f32), block_type);
            }
        }
        chunk
    }
}
