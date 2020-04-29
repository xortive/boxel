use crate::primitives::InstanceAttr;
#[derive(Clone, Debug)]
pub struct Block {
  pub position: (f32, f32, f32),
}

impl Into<InstanceAttr> for Block {
  fn into(self) -> InstanceAttr {
    InstanceAttr {
      world_position: self.position
    }
  }
}