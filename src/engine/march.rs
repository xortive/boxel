use glm::{IVec3, Vec3, vec3};
use nalgebra::base::iter::ColumnIter;

pub struct VoxelMarch {
  pos: IVec3,
  tMax: Vec3,
  tD: Vec3,
  step: IVec3,
}


// Adapted from "A Fast Voxel Traversal Algorithm for Ray Tracing" by John Amanatides and Andrew Woo, 1987
impl VoxelMarch {
  pub fn new(origin: &Vec3, dir: &Vec3) -> Self { 
    let start: IVec3 = glm::try_convert(glm::floor(&origin)).unwrap();
    let tMax = Vec3::from_vec(origin.iter().enumerate().map(|(i, &s)| {
      let t = Self::unit_step(s, dir[i]);
      if !t.is_nan() && !t.is_infinite() { t } else { f32::MAX }
    }).collect());
    let step: IVec3 = glm::try_convert(dir.map(|o| { 
      if o > 0. || o < 0. { o.signum() } else { 0. }
    })).unwrap();
    let tD = Vec3::from_vec(step.iter().enumerate().map(|(i, &s)| {
      let d = s as f32 / dir[i];
      if !d.is_nan() && !d.is_infinite() { d } else { 0. }
    }).collect());

    //println!("March starting at {} in dir {} tmax {} step {} tD {}", start, dir, tMax, step, tD);

    Self {
      pos: start,
      tMax,
      tD,
      step,
    }
  }

  // get the unit T for start, direction such that
  // start + t * dir is the nearest int in the direction of d
  fn unit_step(start: f32, dir: f32) -> f32 {
    (if dir > 0. {
      start.ceil() - start
    } else {
      start - start.floor()
    }) / dir.abs()
  }
}

impl Iterator for VoxelMarch {
  type Item = (IVec3, IVec3);

  //position, normal
  fn next(&mut self) -> Option<(IVec3, IVec3)> {
    let mut normal = vec3(0, 0, 0);
    let dir = self.tMax.imin();
    normal[dir] = -self.step[dir];
    self.pos[dir] += self.step[dir];
    self.tMax[dir] += self.tD[dir];
    //println!("step to {} from {} in {}... tMax {}", self.pos, pos, dir, self.tMax);
    Some((self.pos, normal))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_voxel_march() {
    let mut ray = VoxelMarch::new(&vec3(8., 20., 8.), &glm::normalize(&vec3(-8., -20., -8.)));
    for _ in 0..36 {
      println!("ret to {}", ray.next().unwrap().0);
    }
  }

  #[test]
  fn test_unit_step() {
    for x in -1..=1 {
      for dx in -1..=1 {
        if dx == 0 { continue; }
        let x = x as f32 * 0.5;
        let dx = dx as f32;
        let t = VoxelMarch::unit_step(x,dx);
        let step = t * dx + x;
        println!("x: {} dx: {} t: {} step: {}", x, dx, t, step);
        assert!(step.fract() == 0.0);
      }
    }
  }

  #[test]
  fn test_unit_step_man() {
    dbg!(VoxelMarch::unit_step(8., 0.));
  }

  #[test]
  fn test_nalgebra_imin() {
    //wtf this returns 0
    dbg!(vec3(f32::NAN, f32::NAN, -1.).imin());
  }
}
