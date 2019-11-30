use crate::classes::{ color::Color, vector::Vector };

// MY Implementation of namespaces in rust.

#[derive(Clone, Debug)]
pub struct Surfaces {
  roughness: f64,
  surface_type: SurfaceType,
}

#[derive(Clone, Debug)]
enum SurfaceType {
  Rough,
  Shiny,
  Checkerboard,
}

impl Surfaces {
  fn new(mut self, my_surface_type: SurfaceType) -> Surfaces {
    match my_surface_type {
      SurfaceType::Shiny => {
        self.roughness = 250.0;
        self.surface_type = my_surface_type;
        let surface_type = self.surface_type.clone();
        let roughness = self.roughness.clone();
        Surfaces { surface_type, roughness }
      },
      SurfaceType::Rough => {
        self.roughness = 75.00;
        self.surface_type = my_surface_type;
        let surface_type = self.surface_type.clone();
        let roughness = self.roughness.clone();
        Surfaces { roughness, surface_type }
      },
      SurfaceType::Checkerboard => {
        self.surface_type = my_surface_type;
        self.roughness = 150.00;
        let surface_type = self.surface_type.clone();
        let roughness = self.roughness.clone();
        Surfaces { roughness, surface_type }
      },
    }
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

