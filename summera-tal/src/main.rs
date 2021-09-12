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
    .parse::<usize>().ok().expect("Error"); 
    
    let add_nmr;
    if nmr_times%2 == 0{
        add_nmr = nmr_times/2;
        eprintln!("Is even");
    }else{
        add_nmr = (nmr_times+1)/2-1;
        eprintln!("Is odd");
    }

    eprintln!("{}", add_nmr);

        let mut numbers = lines
            .next().expect("Error")
            .trim()
            .split(" ")
            .map(|component| component.parse::<u32>().ok().expect("Error"))
            .collect::<Vec<u32>>();
    
        numbers.sort();

        let mut result: u32 = 0;

        for _times in add_nmr..nmr_times{
            result = result + numbers[_times];
            eprintln!("{}", numbers[_times])
        }
    
        println!("{}", result);


}