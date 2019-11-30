use std::f64::INFINITY;

pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z, }
  }
  
  pub fn times(k: f64, v: &Vector) -> Vector {
    Vector {
      x: k * v.x,
      y: k * v.y,
      z: k * v.z,
    }
  }

  pub fn minus(v1: &Vector, v2: &Vector) -> Vector {
    Vector {
      x: v1.x - v2.x,
      y: v1.y - v2.y,
      z: v1.z - v2.z,
    }
  }

  pub fn plus(v1: &Vector, v2: &Vector) -> Vector {
    Vector {
      x: v1.x + v2.x,
      y: v1.y + v2.y,
      z: v1.z + v2.z,
    }
  }

  pub fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z 
  }

  pub fn magnitude(v: &Vector) -> f64 {
    ( v.x.powf(2.0) + v.y.powf(2.0) + v.z.powf(2.0) ).sqrt()
  }

  pub fn normalize(v: &Vector) -> Vector {
    let my_magnitude = Vector::magnitude(&v);
    let div: f64;
    if my_magnitude == 0.0 {
      div = INFINITY;
    } else { 
      div = 1.0 / my_magnitude;
    }
    Vector::times(div, v)
  }

  pub fn cross_product(v1: &Vector, v2: &Vector) -> Vector {
    let x: f64 = v1.y * v2.z - v1.z * v2.y;
    let y: f64 = v1.z * v2.x - v1.x * v2.z;
    let z: f64 = v1.x * v2.y - v1.y * v2.x;

    Vector::new(x, y, z)
  }
}