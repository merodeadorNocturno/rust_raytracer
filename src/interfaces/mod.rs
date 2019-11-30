use crate::classes::{
  camera::Camera,
  color::Color,
  vector::Vector,
  surfaces::Surfaces,
  sphere::Sphere,
  plane::Plane,
};

pub struct Ray {
  pub start: Vector,
  pub dir: Vector,
}

pub struct Surface {
  pub roughness: f64,
}

pub struct Intersection<T> {
  pub ray: Ray,
  pub dist: f64,
  pub thing: T,
  pub is_null: bool,
}

pub struct Thing {
  pub surface: Surface,
}

pub trait Intersect<T> {
  fn intersect(self, ray: Ray) -> Intersection<T>;
}

pub struct Light {
  pub pos: Vector,
  pub color: Color,
}

pub struct Scene {
  pub things: Vec<Intersection<(Sphere, Plane, Surfaces)>>,
  pub camera: Camera,
  pub lights: Vec<Light>,
}
