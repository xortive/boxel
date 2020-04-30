#![feature(clamp)]
use crate::primitives::InstanceAttr;

const COLORS: [(f32, f32, f32); 4] = [
  (94./255., 227./255., 230./255.), //blue
  (223./255., 223./255., 160./255.), //tan
  (53./255., 170./255., 70./255.), //green
  (170./255., 170./255., 185./255.), //grey
];

#[derive(Clone, Debug)]
pub struct Block {
  pub position: (f32, f32, f32),
  pub color: usize,
}

impl Into<InstanceAttr> for Block {
  fn into(self) -> InstanceAttr {
    let color = {
      let mut color = self.color;
      if color >= COLORS.len() {
        color = COLORS.len() - 1;
      }
      color
    };

    InstanceAttr {
      world_position: self.position,
      color: COLORS[color],
    }
  }
}