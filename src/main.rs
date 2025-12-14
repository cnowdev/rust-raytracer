use std::io::{self, Write};
pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod hittablelist;
pub mod sphere;
pub mod rtweekend;
pub mod interval;

use color::{Color, write_color};
use ray::Ray;
use vec3::{Vec3, Point3};
use hittable::{Hittable, HitRecord};
use hittablelist::{HittableList};
use rtweekend::{INFINITY, PI};
use crate::{interval::Interval, sphere::Sphere};

fn ray_color(r: &Ray, world: &mut dyn Hittable) -> Color {
    match world.hit(r, Interval::new(0.0, INFINITY)) {
        None => {
            //draw the background
            let unit_direction = Vec3::unit_vector(&r.direction());
            let a = 0.5*(unit_direction.y() + 1.0);
            return (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
        },
        Some(record) => {
            //draw whatever got hit
            return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
        }
    }
}

fn main() {
    //Image Constants

    let aspect_ratio: f64 = 16.0 / 9.0;
    
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;

    let mut world = HittableList::new();

    world.add(Box::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))
    );

    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
    );

    



    // Camera constants

    let focal_length = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = (image_width as f64 / image_height as f64) * viewport_height;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    //Viewport vectors

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //deltas from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width) as f64;
    let pixel_delta_v = viewport_v / (image_height) as f64;

    // upper left pixel location
    let viewport_upper_left = camera_center - Vec3::new(0.0,0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
        
            let pixel_color = ray_color(&r, &mut world);
            write_color(&mut std::io::stdout(), &pixel_color).unwrap();
        }
    }

    eprintln!("\rDone!                        ");
}
