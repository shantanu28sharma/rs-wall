use std::env;
pub mod linear;
pub mod algo;

fn main () {
    let args : Vec<String>= env::args().collect();
    // let pattern = env::args().nth(1).expect("no patter given");
    println!("{:?}", args);
    let procc = linear::Linear::new(args);
    let img = procc.process(); 
    let algo = algo::SciImage::swirl(img, 10.0, 10.0);
    
}