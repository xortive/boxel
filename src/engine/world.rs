use crate::config::RENDER_DISTANCE;
use crate::engine::chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE};
use crate::engine::generator::{PerlinGenerator, WorldGenerator};
use glium::Display;
use glm::Vec3;
use nalgebra::Point2;
use std::collections::HashMap;
use ncollide3d::query::{Ray, RayCast};
use super::march::VoxelMarch;

pub struct World {
    generator: Box<dyn WorldGenerator>,
    chunks: HashMap<ChunkCoordinate, Chunk>,
}

impl World {
    pub fn new() -> World {
        World {
            generator: Box::new(PerlinGenerator::new()),
            chunks: HashMap::new(),
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
        for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let current_chunk = Point2::new(chunk_coord[0] + x, chunk_coord[1] + z);

                if !self.chunks.contains_key(&current_chunk) {
                    println!("Generating chunk {}", current_chunk);
                    let mut chunk = self.generator.generate(current_chunk);
                    chunk.update_visible();
                    self.chunks.insert(current_chunk, chunk);
                } else if let Some(chunk) = self.chunks.get_mut(&current_chunk) {
                    chunk.update_vbo(&display);
                }

            }
        }
    }

    pub fn intersect(&mut self, position: &Vec3, ray: &Ray<f32>) {
        let mut march = VoxelMarch::new(position, &ray.dir);
        for _ in 0..10 {
            let block = march.next().unwrap().0;
            let chunk = Self::convert_to_chunk(&glm::convert(block));
            let chunk = self.chunks.get_mut(&chunk);
            if let Some(chunk) = chunk {
                let mut marcha = VoxelMarch::new(position, &ray.dir);
                chunk.remove(&mut marcha);
                break;
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
                output.push(self.chunks.get(&current_chunk).unwrap())
            }
        }

        output
    }
}
