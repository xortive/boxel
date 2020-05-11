use crate::primitives::InstanceAttr;
use super::types::*;
use std::convert::AsRef;

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    GRASS,
    SAND,
    DIRT,
    STONE,
    WATER,
}

impl BlockType {
    fn color(&self) -> (f32, f32, f32, f32) {
        match *self {
            BlockType::GRASS => (53. / 255., 170. / 255., 70. / 255., 1.),
            BlockType::SAND => (223. / 255., 223. / 255., 160. / 255., 1.),
            BlockType::DIRT => (94. / 255., 227. / 255., 230. / 255., 1.),
            BlockType::STONE => (170. / 255., 170. / 255., 185. / 255., 1.),
            BlockType::WATER => (64. / 255., 164. / 255., 223. / 255., 0.5),
        }
    }
}

#[derive(Clone, Debug)] pub struct Block {
    pub position: WorldCoordinate,
    properties: BlockProperties,
}

impl AsRef<BlockProperties> for Block {
    fn as_ref(&self) -> &BlockProperties {
        &self.properties
    }
}

impl Block {
    pub fn new(position: WorldCoordinate, properties: BlockProperties) -> Block {
        Block {
            position,
            properties
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BlockProperties {
    pub block_type: BlockType,
}

impl Into<InstanceAttr> for Block {
    fn into(self) -> InstanceAttr {
        InstanceAttr {
            world_position: (self.position.x as f32, self.position.y as f32, self.position.z as f32),
            color: self.properties.block_type.color(),
        }
    }
}
