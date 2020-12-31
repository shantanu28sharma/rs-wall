use std::env;
pub mod wall;

fn main () {
    let args : Vec<String>= env::args().collect();
    // let pattern = env::args().nth(1).expect("no patter given");
    println!("{:?}", args);
    let procc = wall::Linear::new(args);
    procc.process(); 
}