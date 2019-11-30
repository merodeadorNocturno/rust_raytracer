use crate::classes::{ color::Color, vector::Vector };

trait SurfaceMethods {
  fn diffuse(pos: Vector) -> Color;
  fn specular(pos: Vector) -> Color;
  fn reflect(pos: Vector) -> f64;
}

trait Shiny: SurfaceMethods {
  fn shiny(pos: Vector) -> Surfaces;
  // fn checkerboard(pos: Vector) -> Surfaces;
}

#[derive(Clone, Debug)]
pub struct Surfaces {
  pub roughness: f64
}

#[derive(Clone, Debug)]
enum SurfaceType {
  Rough,
  Shiny,
  Checkerboard,
}

impl Surfaces {

  pub fn shiny(pos: Vector) -> Surfaces {
    pub fn diffuse(pos) -> SurfaceMethods
  }


  fn diffuse(&self, pos: &Vector) -> Color {
    match self.surface_type {
      SurfaceType::Shiny => Color::white(),
      SurfaceType::Rough => Color::grey(),
      SurfaceType::Checkerboard => {
        if pos.z.floor() as i64 + pos.x.floor() as i64 % 2 != 0 {
          Color::white()
        } else {
          Color::black()
        }
      }
    }
  }

  fn specular(&self, pos: &Vector) -> Color {
    match self.surface_type {
      SurfaceType::Shiny => Color::grey(),
      SurfaceType::Rough => Color::white(),
      SurfaceType::Checkerboard => Color::white(),
    }
  }

  fn reflect(&self, pos: &Vector) -> f64 {
    match self.surface_type {
      SurfaceType::Shiny => 0.7,
      SurfaceType::Rough => 0.24,
      SurfaceType::Checkerboard => {
        if pos.z.floor() as i64 + pos.x.floor() as i64 % 2 != 0 {
          0.10
        } else {
          0.7
        }
      }
    }
  }
}
