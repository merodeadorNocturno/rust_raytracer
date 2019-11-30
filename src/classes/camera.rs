use crate::classes::vector::Vector;

pub struct Camera {
  forward: Vector,
  right: Vector,
  up: Vector,
}

impl Camera {
  pub fn new(pos: &Vector, look_at: &Vector) -> Camera {
    let down = Vector::new(0.0, -1.0, 0.0);

    let forward = Vector::normalize(&Vector::minus(&look_at, &pos));
    let right = Vector::times(
      1.5,
      &Vector::normalize(&Vector::cross_product(&forward, &down)),
    );
    let up = Vector::times(
      1.5,
      &Vector::normalize(&Vector::cross_product(&forward, &right)),
    );

    Camera { forward, right, up }
  }
}
