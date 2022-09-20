use std::io;

fn main() {
    let mut s  = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
    
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

