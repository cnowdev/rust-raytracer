use std::io::{self, Write};
pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod hittablelist;
pub mod sphere;
pub mod rtweekend;
pub mod interval;
pub mod camera;

use vec3::{Point3};
use hittablelist::{HittableList};
use sphere::Sphere;
use camera::Camera;
fn main() {
    let mut world = HittableList::new();

    world.add(Box::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))
    );
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
    );

    let camera: Camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);

}
