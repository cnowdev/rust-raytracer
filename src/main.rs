pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod hittablelist;
pub mod sphere;
pub mod rtweekend;
pub mod interval;
pub mod camera;
pub mod material;

use vec3::{Point3};
use hittablelist::{HittableList};
use sphere::Sphere;
use camera::Camera;
use color::Color;
use material::{Lambertian};
use std::sync::Arc;
fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    let material_left = Arc::new(material::Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, center_material.clone()))
    );
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground_material.clone()))
    );

    world.add(Box::new(
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone()))
    );
    world.add(Box::new(
        Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone()))
    );

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let samples_per_pixel: u16 = 100;
    let max_depth: u16 = 50;

    let camera: Camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    camera.render(&world);

}
