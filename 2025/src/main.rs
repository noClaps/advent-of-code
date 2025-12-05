use std::env;

mod p1;
mod p2;
mod p3;
mod p4;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args[1].as_str() {
        "1" => p1::p1(),
        "2" => p2::p2(),
        "3" => p3::p3(),
        "4" => p4::p4(),
        _ => panic!("Invalid problem number"),
    };
}
