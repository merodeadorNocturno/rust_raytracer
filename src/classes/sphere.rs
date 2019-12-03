use crate::classes::vector::Vector;
use crate::interfaces::{ Ray, Surface, Intersection, Intersect };

pub struct Sphere {
  pub center: Vector,
  pub radius2: f64,
  pub surface: Surface,
}

impl Sphere {
  pub fn new(mut self, center: Vector, radius: f64, surface: Surface) {
    self.center = center;
    self.radius2 = radius.powf(2.0);
    self.surface = surface;
  }

  fn normalize(self, pos: Vector) -> Vector {
    let my_center = self.center;
    Vector::normalize(&Vector::minus(&pos, &my_center))
  }
}

impl Intersect<Sphere> for Sphere {
  fn intersect(self, ray: Ray) -> Intersection<Self> {
    let eo:Vector = Vector::minus(&self.center, &ray.start);
    let v = Vector::dot(&eo, &ray.dir);
    let mut dist = 0.0;
    let mut is_null: bool = false;

    if v >= 0.0 {
      let disc = self.radius2 - (Vector::dot(&eo, &eo) - v.powf(2.0));

      if disc >= 0.0 {
        dist = v - disc.sqrt();
      }
    }

    if dist == 0.0 {
      is_null = true;
    }

    Intersection{ thing: self, ray, dist, is_null }
  }
}
