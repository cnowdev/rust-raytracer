use crate::vec3::{Vec3, Point3};
use crate::color::{write_color, Color};
use crate::ray::{self, Ray};
use crate::hittable::Hittable;
use crate::{interval::Interval};
use std::io::{self, Write};
use crate::rtweekend::{INFINITY, random_double};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u16,
    pub samples_per_pixel: u16,
    pub depth: u16,
    image_height: u16,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) -> () {

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:3}", self.image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..self.samples_per_pixel {
                    let r = Self::get_ray(&self, i, j);
                    pixel_color = pixel_color + Self::ray_color(&r, self.depth, world);
                }
                write_color(&mut std::io::stdout(), &(self.pixel_samples_scale * pixel_color)).unwrap();
            }
        }

        eprintln!("\rDone!                        ");
    }

    pub fn new(aspect_ratio: f64, image_width: u16, samples_per_pixel: u16, depth: u16) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as u16;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

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
            samples_per_pixel,
            depth,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(r: &Ray, depth: u16, world: &dyn Hittable) -> Color  {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(r, Interval::new(0.001, INFINITY)) {
            None => {
                //draw the background
                let unit_direction = Vec3::unit_vector(&r.direction());
                let a = 0.5*(unit_direction.y() + 1.0);
                return (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
            },
            Some(record) => {
                //draw whatever got hit
                match record.mat.scatter(r, &record) {
                    None => return Color::new(0.0, 0.0, 0.0),
                    Some((attenuation, scattered)) => {
                        return attenuation * Self::ray_color(&scattered, depth-1, world);
                    }
                }
            }
        }
    }

    //Make a ray that comes from camera to a randomly sampled point around the pixel loc i,j
    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + 
                                 self.pixel_delta_u * (i as f64 + offset.x()) +
                                 self.pixel_delta_v * (j as f64 + offset.y());
        
        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vec3 {
        //random point in the square [-0.5, -0.5] x [-0.5, 0.5]
        return Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0);
    }
}