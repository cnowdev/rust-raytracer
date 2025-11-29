use std::io::{self, Write};
pub mod vec3;
pub mod color;

fn main() {
    let image_height: u16 = 256;
    let image_width: u16 = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_color = color::Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            color::write_color(&mut std::io::stdout(), &pixel_color).unwrap();
        }
    }

    eprintln!("\rDone!                        ");
}
