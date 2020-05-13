use crate::primitives::InstanceAttr;

#[derive(Clone, Debug, PartialEq)]
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
            BlockType::WATER => (64. / 255., 164. / 255., 223. / 255., 0.95),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub position: (f32, f32, f32), // in world space
    pub block_type: BlockType,
}

impl Block {
    pub fn globalToLocal(&self, v: &glm::Vec4) -> glm::Vec3 {
        let translate = glm::translation(&glm::vec3(self.position.0, self.position.1, self.position.2));
        let inverse = glm::inverse(&translate);
        let result = inverse * v;
        glm::vec3(result[0], result[1], result[2])
    }
}

impl Into<InstanceAttr> for Block {
    fn into(self) -> InstanceAttr {
        InstanceAttr {
            world_position: self.position,
            color: self.block_type.color(),
        }
    }
}
