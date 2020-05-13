use crate::config::RENDER_DISTANCE;
use crate::engine::chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE, IntersectAction};
use crate::engine::generator::{PerlinGenerator, WorldGenerator};
use glium::Display;
use glm::Vec3;
use nalgebra::Point2;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use super::march::VoxelMarch;

pub struct World {
    generator: Arc<Mutex<dyn WorldGenerator + Send>>,
    chunks: HashMap<ChunkCoordinate, Chunk>,
    chunk_queue: Arc<Mutex<Vec<Chunk>>>,
    generated: Vec<ChunkCoordinate>,
}

impl World {
    pub fn new(seed: u32) -> World {
        World {
            generator: Arc::new(Mutex::new(PerlinGenerator::new(seed))),
            chunks: HashMap::new(),
            chunk_queue: Arc::new(Mutex::new(Vec::new())),
            generated: Vec::new(),
        }
    }

    fn convert_to_chunk(position: &Vec3) -> ChunkCoordinate {
        Point2::new(
            (position[0] / CHUNK_SIZE as f32) as i32,
            (position[2] / CHUNK_SIZE as f32) as i32,
        )
        // glm::vec2((position[0] / CHUNK_SIZE as f32).floor(), (position[2] / CHUNK_SIZE as f32).floor())
    }

    pub fn update_chunks(&mut self, position: &Vec3, display: &Display) {
        let chunk_coord = Self::convert_to_chunk(&position);

        for chunk in self.chunk_queue.lock().unwrap().drain(0..) {
            self.chunks.insert(chunk.coordinates, chunk);
        }

        for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let current_chunk = Point2::new(chunk_coord[0] + x, chunk_coord[1] + z);

                if !self.chunks.contains_key(&current_chunk) && !self.generated.contains(&current_chunk) {
                    println!("Generating chunk {}", current_chunk);
                    self.generated.push(current_chunk);
                    let queue = self.chunk_queue.clone();
                    let generator = self.generator.clone();
                    thread::spawn(move || {
                        let mut chunk = generator.lock().unwrap().generate(current_chunk);
                        chunk.update_visible();
                        queue.lock().unwrap().push(chunk);
                    });
                    // self.chunks.insert(current_chunk, chunk);
                } else if let Some(chunk) = self.chunks.get_mut(&current_chunk) {
                    chunk.update_vbo(&display);
                }

            }
        }
    }

    pub fn intersect(&mut self, position: &Vec3, ray: &Vec3, action: &IntersectAction) {
        let mut march = VoxelMarch::new(position, &ray);
        for _ in 0..200 {
            let block = march.next().unwrap().0;
            let chunk = Self::convert_to_chunk(&glm::convert(block));
            //println!("collide {}", chunk);
            let chunk = self.chunks.get_mut(&chunk);
            if let Some(chunk) = chunk {
                if chunk.intersect(&mut march, &action) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn rendered_chunks(&self, position: &Vec3) -> Vec<&Chunk> {
        let chunk_coord = Self::convert_to_chunk(&position);
        let mut output = Vec::new();
        for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let current_chunk = Point2::new(chunk_coord[0] + x, chunk_coord[1] + z);
                if let Some(chunk) = self.chunks.get(&current_chunk) {
                    output.push(chunk);
                }
                // output.push(self.chunks.get(&current_chunk).unwrap())
            }
        }

        output
    }
}
