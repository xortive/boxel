use super::block::{Block, BlockFace, BlockProperties, BlockType};
use super::types::*;
use crate::config::*;
use crate::primitives::InstanceAttr;
use glium::vertex::PerInstance;
use glium::{Display, VertexBuffer};
use glm::{vec3, IVec3, UVec3};

use std::convert::TryFrom;

pub struct Chunk {
    coordinates: ChunkCoordinate, //in chunk space, so (0, 0) is the chunk from worldspace (0,y,0) to (16,y,16);
    //blocks[x][z][y]
    blocks: ChunkStorage,
    vbo: Option<VertexBuffer<InstanceAttr>>,
}

pub struct ChunkStorage {
    blocks: [[[Option<BlockProperties>; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize];
        CHUNK_SIZE as usize],
    visible: Option<[[[bool; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize];
        CHUNK_SIZE as usize]>,
}

impl ChunkStorage {
    pub fn new() -> Self {
        Self {
            blocks: [[[None; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
            visible: None
        }
    }

    pub fn get_props(&self, block: BlockCoordinate) -> Option<BlockProperties> {
        self.blocks[block.x as usize][block.z as usize][block.y as usize]
    }

    pub fn is_visible(&mut self, block: BlockCoordinate) -> bool {
        if self.visible.is_none() {
            self.update_visible();
        }
        self.visible.unwrap()[block.x as usize][block.z as usize][block.y as usize]
    }


    pub fn update_visible(&mut self) {
        let mut visible = [[[false; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        println!("Calculating visibility...");

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {

                let mut air = true;

                for y in 0..CHUNK_HEIGHT {
                    let block = self.get_props(vec3(x,y,z));

                    //if we were in air, and block is solid, block is visible
                    if air && block.is_some() {
                        visible[x as usize][z as usize][y as usize] = true;
                    }
                   
                    //if we were in a block, and we are going into air, last block is visible
                    if !air && block.is_none() {
                        if let Ok(y) = usize::try_from(y-1) {
                            visible[x as usize][z as usize][y] = true;
                        }
                    }

                    //we still aren't visible, let's check our horizontal neighbors
                    //TODO implement

                    air = block.is_none();
                }
            }
        }
        self.visible = Some(visible);

        println!("done!");
    }

    pub fn set(&mut self, block: BlockCoordinate, props: Option<BlockProperties>) {
        self.blocks[block.x as usize][block.z as usize][block.y as usize] = props;
        self.visible = None
    }

    pub fn set_with_visibility(&mut self, block: BlockCoordinate, props: Option<BlockProperties>, visible: bool) {
        self.blocks[block.x as usize][block.z as usize][block.y as usize] = props;
        let mut vis = self.visible.unwrap_or([[[false; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]);
        vis[block.x as usize][block.z as usize][block.y as usize] = visible;
        self.visible = Some(vis)
    }
}

impl Chunk {
    pub fn new(coordinates: ChunkCoordinate) -> Chunk {
        let c = Chunk {
            coordinates,
            blocks: ChunkStorage::new(),
            vbo: None,
        };
        // c.add_plane();
        c
    }

    pub fn get(&self, block: BlockCoordinate) -> Option<Block> {
        self.blocks.get_props(block).map(|props| {
            let bi: WorldCoordinate = glm::convert(block);
            Block::new(self.world_origin() + bi, props)
        })
    }


    pub fn add_block(&mut self, coordinate: BlockCoordinate, block_type: BlockType) {
        //println!("coords {:?}", coordinate);
        self.blocks
            .set(coordinate, Some(BlockProperties { block_type }));
    }

    pub fn add_block_with_visibility(&mut self, coordinate: BlockCoordinate, block_type: BlockType, visible: bool) {
        //println!("coords {:?}", coordinate);
        self.blocks
            .set_with_visibility(coordinate, Some(BlockProperties { block_type }), visible);
    }

    pub fn world_origin(&self) -> WorldCoordinate {
        let world = self.coordinates * CHUNK_SIZE as i32;
        vec3(world[0], 0, world[1])
    }

    pub fn update_vbo(&mut self, display: &Display) {
        let mut instances: Vec<InstanceAttr> = Vec::new();
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HEIGHT {
                    let coords = vec3(x, y, z);
                    if let Some(block) = self.get(vec3(x, y, z)) {
                        if self.blocks.is_visible(coords) {
                            instances.push(block.into());
                        }
                    }
                }
            }
        }

        println!("{} instances", instances.len());

        let vbo = VertexBuffer::new(display, &instances).expect("to create vb");

        self.vbo = Some(vbo);
    }

    //loads instance if they don't exist
    pub fn per_instance(&self) -> PerInstance {
        self.vbo.as_ref().unwrap().per_instance().unwrap()
    }
}
