use palette::{FromColor, Hsv, IntoColor, Lab, LinSrgb, Pixel, Shade, Srgb, Gradient};

use image::{GenericImage, GenericImageView, RgbImage};

fn main() {
    //The same color in linear RGB, CIE L*a*b*, and HSV
    let rgb = Srgb::new(0.5, 0.0, 0.0).into_linear();
    let lab = Lab::from_rgb(rgb);
    let hsv = Hsv::from(rgb);

    let mut img = RgbImage::new(500, 500);

    let grad = Gradient::new(vec![
        (LinSrgb::new(1.0, 0.1, 0.1)),
        (LinSrgb::new(0.1, 1.0, 1.0))
    ]);

    let (w, h) = img.dimensions();
    let mut iter = grad.take(10);
    let mut val = iter.next().unwrap();

    println!("{:?}", val);
    let mut col: [f32; 3] = Srgb::into_raw(Srgb::from_linear(val));
    println!("{:?}", col);
    
    for x in 0..w {
        for y in 0..h {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([(col[0] *255.0) as u8, (col[1] *255.0) as u8, (col[2] *255.0) as u8]);
        }
        if (x+1)%(w/10) == 0 {
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