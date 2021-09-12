use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());


    let nmr_times = lines
        .next().unwrap()
        .trim()
        .parse::<usize>().ok().unwrap(); 

    eprintln!("\n number of numbers: {}\n", nmr_times);

    
    let add_nmr;
    if nmr_times%2 == 0{
        add_nmr = nmr_times/2;
    }else{
        add_nmr = (nmr_times+1)/2;
    }

    eprintln!("\n number of adding numbers: {}\n", add_nmr);



        let mut numbers = lines
            .next().unwrap()
            .split(" ")
            .map(|component| component.parse::<u32>().ok().unwrap())
            .collect::<Vec<u32>>();
    
        numbers.sort();

        eprintln!("\n The numbers are: {:?} \n", numbers);

        let mut result: u32 = 0;

        for _times in (add_nmr-1)..nmr_times{
            result = result + numbers[_times];
        }
    
        println!("{}", result);


}