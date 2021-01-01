use palette::{LinSrgb, Pixel, Srgb, Gradient};
use image::{RgbImage};
use raster::Color;
use std::error;
use std::f32::consts;

pub struct SciImage;

impl SciImage {
    pub fn swirl(img: RgbImage, strength: f32, radius: f32) -> Result<RgbImage, Box<dyn error::Error>>{
        let (w, h) = img.dimensions();
        let mut f_img = RgbImage::new(w, h);
        let x0 : f32 = 0.5*(w as f32 - 1.0);
        let y0 : f32 = 0.5*(h as f32 - 1.0);  
        for x in 0..w {
            for y in 0..h {
                let f_x = x as f32;
                let f_y = y as f32;
                let dx = f_x - x0;
                let dy = f_y - x0;
                let r = (dx*dx + dy*dy).sqrt();
                let angle = consts::PI/256.0*r;
                let u = (dx * angle.cos() - dy*angle.sin() + x0) as u32;
                let v = (dx * angle.sin() - dy*angle.cos() + y0) as u32;
                if u < w  && v < h {
                    let pixel = img.get_pixel(u, v);
                    f_img.put_pixel(x, y, *pixel);
                }
            }
        }
        // let angle = strength*(())
        let _ = std::fs::create_dir("example-data/output");
        match f_img.save("example-data/output/shade.png") {
            Ok(()) => println!("see 'example-data/output/shade.png' for the result"),
            Err(e) => println!("failed to write 'example-data/output/shade.png': {}", e),
        };
        Ok(f_img)
    }
}