use crate::classes::vector::Vector;
use crate::interfaces::{ Ray, Surface, Intersection, Intersect };

pub struct Plane {
  norm: Vector,
  offset: f64,
}

impl Plane {
  pub fn new(mut self, norm: &Vector, offset: f64, surface: Surface) {
    self.norm = Vector::new(norm.x, norm.y, norm.z);
    self.offset = offset;
  }

  fn normalize(self, pos: Vector) -> Vector {
    self.norm
  }
}

impl Intersect<Plane> for Plane {
  fn intersect(self, ray: Ray) -> Intersection<Self> {
    let denom = Vector::dot(&self.norm, &ray.dir);
    let dist = (Vector::dot(&self.norm, &ray.start) + self.offset) / (-denom);
    let mut is_null: bool = false;

    if denom > 0.0 {
      is_null = true;
    }

    Intersection { thing: self, ray, dist, is_null }
  }
}