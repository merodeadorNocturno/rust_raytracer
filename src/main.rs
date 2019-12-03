use std::f64::INFINITY;

mod classes;
mod interfaces;

use classes::camera::Camera;
use classes::color::Color;
use classes::vector::Vector;
use classes::sphere::Sphere;
use classes::plane::Plane;

use crate::interfaces::{ Intersection, Ray, Scene };

fn intersections(ray: Ray, scene: Scene) -> Intersection<(Sphere, Plane)> {
  let mut closest = INFINITY;
  let mut closestInter: Intersection<(Sphere, Plane)>;

  for i in scene.things {
    let inter = i.thing.intersect(i.thing, ray);
    if inter.is_null &&  inter.dist < closest {
      closestInter = inter;
      closest = inter.dist;
    }
  }

  closestInter
}

fn main() {
  let max_depth: f64 = 5.0;
}
