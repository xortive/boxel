use crate::engine::block::{Block, BlockType};
use crate::primitives::InstanceAttr;
use glium::vertex::PerInstance;
use glium::{Display, VertexBuffer};
use nalgebra::{Point2, Point3};
use crate::config::HEIGHT_OFFSET;
use std::collections::HashSet;
use std::collections::HashMap;
use super::march::VoxelMarch;

use glm::{IVec3, vec3};

pub const CHUNK_SIZE: i32 = 16;

pub type ChunkCoordinate = Point2<i32>; // chunk space
pub type BlockCoordinate = Point3<i32>; // block space in a chunk (CHUNK_LENGTH x CHUNK_WIDTH x CHUNK_HEIGHT)

pub struct Chunk {
    coordinates: ChunkCoordinate, //in chunk space, so (0, 0) is the chunk from worldspace (0,y,0) to (16,y,16);
    blocks: HashMap<BlockCoordinate, Block>, 
    visible: HashSet<BlockCoordinate>,
    vbo: Option<VertexBuffer<InstanceAttr>>,
}

impl Chunk {
    pub fn new(coordinates: ChunkCoordinate) -> Chunk {
        let c = Chunk {
            coordinates,
            blocks: HashMap::new(),
            visible: HashSet::new(),
            vbo: None,
        };
        // c.add_plane();
        c
    }

    fn get_adjacent(&self, coordinate: &BlockCoordinate) -> Vec<BlockCoordinate> {
        let mut adjacent: Vec<BlockCoordinate>  = vec![];
        for face in 1..=3 {
            for offset in [-1, 1].iter() {
                // let off = *offset;
                adjacent.push([coordinate[0] + (if face == 1 {*offset} else {0}),
                    coordinate[1] + (if face == 2 {*offset} else {0}),
                    coordinate[2] + (if face == 3 {*offset} else {0})].into());
            }
        }
        // println!("For coord: {}", coordinate);
        // for adj in adjacent.iter() {
        //     println!("Adj {}", adj);
        // }
        // for x in -1..=1 {
        //     for y in -1..=1 {
        //         for z in -1..=1 {
        //             if x == 0 && y == 0 && z == 0 { continue; }
        //             adjacent.push([coordinate[0] + x, coordinate[1] + y, coordinate[2] + z].into());
        //         }
        //     }
        // }
        adjacent
    }

    pub fn update_visible(&mut self) {
    //    for block in self. 
        println!("updating visible");
        for (coordinate, block) in self.blocks.iter() {
            // if self.on_edge(coordinate) {
            //     println!("on an edge {}", coordinate);
            //     self.visible.insert(*coordinate);
            //     continue;
            // }

            if block.block_type == BlockType::WATER {
                self.visible.insert(*coordinate);
                continue;
            }

            let mut visible = false;
            for adjacent in self.get_adjacent(coordinate) {
                // if self.on_edge(&adjacent) {
                //     continue;
                // }

                match self.blocks.get(&adjacent) {
                    None => {
                        // println!("Found none: {}", adjacent);
                        visible = true;
                        break;
                    },
                    Some(b) => {
                        if b.block_type == BlockType::WATER {
                            visible = true;
                            break;
                        }
                    }
                }
            }

            if visible {
                self.visible.insert(*coordinate);
            }
        }
        println!("Visible size: {}", self.visible.len());
        println!("finished updating visible");
    }

    pub fn add_block(&mut self, coordinate: BlockCoordinate, block_type: BlockType) {
        // first, convert to world space
        let origin = self.world_origin();

        let world_space: BlockCoordinate = [origin[0] + coordinate[0], origin[1] + coordinate[1], origin[2] + coordinate[2]].into();
        self.blocks.insert(coordinate, Block {
            position: (world_space[0] as f32, world_space[1] as f32 + (HEIGHT_OFFSET as f32), world_space[2] as f32),
            block_type: block_type,
        });

        // self.blocks.push(Block {
        //     position: (world_space[0], world_space[1] + (HEIGHT_OFFSET as f32), world_space[2]),
        //     block_type: block_type,
        // });
    }

    pub fn world_origin(&self) -> Point3<i32> {
        let world = self.coordinates * 16;
        [world[0], 0, world[1]].into()
    }

    fn get_rendered(&self) -> Vec<InstanceAttr> {
        // TODO only get visible
        self.blocks.clone().into_iter().filter(|(pos, _)| self.visible.contains(&pos)).map(|(_, b)| b.into()).collect()
    }

    fn in_chunk(&self, block: &BlockCoordinate) -> bool {
        (0..CHUNK_SIZE).contains(&block[0]) &&
        (0..CHUNK_SIZE).contains(&block[2])
    }

    fn to_chunk_coords(&self, block: &IVec3) -> BlockCoordinate {
        let origin = self.coordinates * CHUNK_SIZE;
        let block = block - vec3(origin.x, 0, origin.y);
        block.into()
    }

    pub fn remove(&mut self, ray: &mut VoxelMarch) -> bool {
        loop {
            let block = ray.next().unwrap().0;
            let pos = self.to_chunk_coords(&block);
            if !self.in_chunk(&pos) { return false } else {
                if self.blocks.contains_key(&pos) {
                    println!("remove {}", block);
                    self.blocks.remove(&pos);
                    self.vbo = None;
                    return true
                }
            }
        }
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
        if self.vbo.is_none() {
            let instances: Vec<InstanceAttr> = self.get_rendered();

            println!("{} instances", instances.len());

            let vbo = VertexBuffer::new(display, &instances).expect("to create vb");

            self.vbo = Some(vbo);
        }
    }

    //loads instance if they don't exist
    pub fn per_instance(&self) -> Option<PerInstance> {
        self.vbo.as_ref().and_then(|vbo| vbo.per_instance().ok())
    }
}
