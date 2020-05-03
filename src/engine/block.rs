use crate::primitives::InstanceAttr;

#[derive(Clone, Debug)]
pub enum BlockType {
    GRASS,
    SAND,
    DIRT,
    STONE,
}

impl BlockType {
    fn color(&self) -> (f32, f32, f32) {
        match *self {
            BlockType::GRASS => (53. / 255., 170. / 255., 70. / 255.),
            BlockType::SAND => (223. / 255., 223. / 255., 160. / 255.),
            BlockType::DIRT => (94. / 255., 227. / 255., 230. / 255.),
            BlockType::STONE => (170. / 255., 170. / 255., 185. / 255.),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub position: (f32, f32, f32),
    pub block_type: BlockType,
}

impl Into<InstanceAttr> for Block {
    fn into(self) -> InstanceAttr {
        InstanceAttr {
            world_position: self.position,
            color: self.block_type.color(),
        }
    }
}
