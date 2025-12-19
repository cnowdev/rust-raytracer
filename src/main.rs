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

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let samples_per_pixel: u16 = 100;

    let camera: Camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);
    camera.render(&world);

}
