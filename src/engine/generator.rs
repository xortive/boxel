use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, ChunkCoordinate};

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

pub struct PerlinGenerator {}

impl WorldGenerator for PerlinGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk {
        Chunk::new(coordinate)
    }
}
