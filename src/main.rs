use std::env;
pub mod linear;

fn main () {
    let args : Vec<String>= env::args().collect();
    // let pattern = env::args().nth(1).expect("no patter given");
    println!("{:?}", args);
    let procc = linear::Linear::new(args);
    procc.process(); 
}