use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    for _line in input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap()) {

        let mut numbers = _line.split_whitespace();
        
        let num1:u64 = numbers.next().expect("Error").trim().parse().expect("Error");
        let num2:u64 = numbers.next().expect("Error").trim().parse().expect("Error");
        
        if num1 >= num2{
            println!("\n{}", (num1 - num2));
        }else{
            println!("\n{}", (num2 - num1));
        }
        
    }
}
