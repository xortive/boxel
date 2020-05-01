use crate::engine::generator::{WorldGenerator, PlanarGenerator};
use crate::engine::chunk::{ChunkCoordinate, Chunk, CHUNK_SIZE};
use glium::{Display};
use nalgebra::Point2;
use crate::config::RENDER_DISTANCE;
use glm::Vec3;
use std::collections::HashMap;

pub struct World {
   generator: Box<dyn WorldGenerator>,
   chunks: HashMap<ChunkCoordinate, Chunk>
}

impl World {
    pub fn new() -> World {
        World {
            generator: Box::new(PlanarGenerator::new()),
            chunks: HashMap::new()
        }
    }

    fn convert_to_chunk(&self, position: &Vec3) -> ChunkCoordinate {
        Point2::new((position[0] / CHUNK_SIZE as f32) as i32, (position[2] / CHUNK_SIZE as f32) as i32)
        // glm::vec2((position[0] / CHUNK_SIZE as f32).floor(), (position[2] / CHUNK_SIZE as f32).floor())
    }

    pub fn update_chunks(&mut self, position: &Vec3, display: &Display) {
        let chunk_coord = self.convert_to_chunk(&position);
        for x in -RENDER_DISTANCE..RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..RENDER_DISTANCE {

                let current_chunk = Point2::new(chunk_coord[0] + x, chunk_coord[1] + z);

                if !self.chunks.contains_key(&current_chunk) {
                    println!("Generating chunk {}", current_chunk);
                    let mut chunk = self.generator.generate(current_chunk);
                    chunk.update_vbo(display);
                    self.chunks.insert(current_chunk, chunk);
                }
            }
        }
    }

    pub fn rendered_chunks(&self, position: &Vec3) -> Vec<&Chunk> {
        let chunk_coord = self.convert_to_chunk(&position);
        let mut output = Vec::new();
        for x in -RENDER_DISTANCE..RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..RENDER_DISTANCE {
                let current_chunk = Point2::new(chunk_coord[0] + x, chunk_coord[1] + z);
                output.push(self.chunks.get(&current_chunk).unwrap())
            }
        }

        output
    }
}