use std::env;

mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args[1].as_str() {
        "1" => p1::p1(),
        "2" => p2::p2(),
        "3" => p3::p3(),
        "4" => p4::p4(),
        "5" => p5::p5(),
        "6" => p6::p6(),
        _ => panic!("Invalid problem number"),
    };
}
