use crate::vec3::{Vec3, Point3};
use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::{interval::Interval, sphere::Sphere};
use std::io::{self, Write};
use crate::rtweekend::{INFINITY, PI};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u16,   
    image_height: u16,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) -> () {

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + self.pixel_delta_u * i as f64 + self.pixel_delta_v * j as f64;
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
            
                let pixel_color = Self::ray_color(&r, world);
                write_color(&mut std::io::stdout(), &pixel_color).unwrap();
            }
        }

        eprintln!("\rDone!                        ");
    }

    pub fn new(aspect_ratio: f64, image_width: u16) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as u16;
        let center = Point3::new(0.0, 0.0, 0.0);

        // Find viewport dimensions
        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = (image_width as f64 / image_height as f64) * viewport_height;

        // Find vectors across the horizontal and down vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Find the horizontal and vertical delta vectors for pixel-to-pixel spacing
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Location of the upper left pixel
        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color  {
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
}