use palette::{LinSrgb, Pixel, Srgb, Gradient};
use image::{RgbImage};
use raster::Color;

pub struct Linear {
    div: u32,
    color1: String,
    color2: String,
    w: u32,
    h: u32,
    outline: String
}

impl Linear {
    pub fn new(args: Vec<String>) -> Self {
        // div: u32, color1: String, color2: String, w: u32, h: u32
        Linear{
            div: args[1].parse().unwrap(),
            color1: args[2].to_owned(),
            color2: args[3].to_owned(),
            w: 500,
            h: 500,
            outline: args[4].to_owned()
        }
    }
    pub fn process(&self){
        let mut img = RgbImage::new(self.w, self.h);
        let color1 = Color::hex(&self.color1).unwrap();
        let color2 = Color::hex(&self.color2).unwrap();
        let outline = Color::hex(&self.outline).unwrap();
        let grad = Gradient::new(vec![
            LinSrgb::new((color1.r as f32)/255.0, (color1.g as f32)/255.0, (color1.b as f32)/255.0),
            LinSrgb::new((color2.r as f32)/255.0, (color2.g as f32)/255.0, (color2.b as f32)/255.0)
        ]);
        let (w, h) = (self.h, self.w);
        let mut iter = grad.take(10);
        let mut val = iter.next().unwrap();
        let mut col: [f32; 3] = Srgb::into_raw(Srgb::from_linear(val));
        
        for x in 0..w {
            for y in 0..h {
                let pixel = img.get_pixel_mut(x, y);
                if x%(w/self.div) == w/self.div-1 || y%(h/self.div) == h/self.div-1 {
                    *pixel = image::Rgb([0, 0, 0]);
                    continue;
                }
                *pixel = image::Rgb([(col[0] *255.0) as u8, (col[1] *255.0) as u8, (col[2] *255.0) as u8]);
            }
            if (x+1)%(w/self.div) == 0 {
                match iter.next(){
                    Some(v) => {
                        val = v;
                        col = Srgb::into_raw(Srgb::from_linear(val));
                    },
                    None => { }
                }
            }
        }
        
        let _ = std::fs::create_dir("example-data/output");
        match img.save("example-data/output/shade.png") {
            Ok(()) => println!("see 'example-data/output/shade.png' for the result"),
            Err(e) => println!("failed to write 'example-data/output/shade.png': {}", e),
        }
    }
}