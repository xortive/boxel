use crate::engine::chunk::{Chunk, ChunkCoordinate};
use crate::engine::block::BlockType;

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
                chunk.add_block(glm::vec3(x as f32, 0., z as f32), BlockType::DIRT);
            }
        }
        chunk
    }
}

pub struct PerlinGenerator {

}

impl WorldGenerator for PerlinGenerator {
    fn generate(&self, coordinate: ChunkCoordinate) -> Chunk {
        Chunk::new(coordinate)
    }
}