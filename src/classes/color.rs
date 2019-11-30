pub struct Color {
  r: f64,
  g: f64,
  b: f64,
}

impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
  }

  pub fn scale(k: f64, c: Color) -> Color {
    Color {
      r: k * c.r,
      g: k * c.g, 
      b: k * c.b
    }
  }

  pub fn plus(c1: Color, c2: Color) -> Color {
    Color::new(
      c1.r + c2.r,
      c1.g + c2.g,
      c1.b + c1.b
    )
  }

  pub fn times(c1: Color, c2:Color) -> Color {
    Color::new(
      c1.r * c2.r,
      c1.g * c2.g,
      c1.b * c2.b
    )
  }

  pub fn white() -> Color {
    Color::new(1.0, 1.0, 1.0)
  }

  pub fn grey() -> Color {
    Color::new(0.5, 0.5, 0.5)
  }

  pub fn black() -> Color {
    Color::new(0.0, 0.0, 0.0)
  }

  pub fn background() -> Color {
    Color::black()
  }

  pub fn default_color() -> Color {
    Color::black()
  }

  pub fn to_drawing_color(c: Color) -> Color {
    let legalize = |d| {
      if d > 1.0 {
        1.0
      } else {
        d
      }
    };
    Color::new(
      legalize(c.r).round() * 255.0,
      legalize(c.g).round() * 255.0,
      legalize(c.b).round() * 255.0,
    )
  }
}

