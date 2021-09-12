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


    let nmrTimes = lines
        .next().unwrap()
        .parse::<usize>().unwrap(); 

    eprintln!("\n number of numbers: {}\n", nmrTimes);

    
    let addNmr;
    if nmrTimes%2 == 0{
        addNmr = nmrTimes/2;
    }else{
        addNmr = (nmrTimes+1)/2;
    }

    eprintln!("\n number of adding numbers: {}\n", addNmr);



    for _case in 0..nmrTimes{

        let mut numbers = lines
            .next().unwrap()
            .split(" ")
            .map(|component| component.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    
        numbers.sort();

        eprintln!("\n The numbers are: {:?} \n", numbers);

        let mut result: usize = 0;

        for _times in (addNmr-1)..nmrTimes{
            result = result + numbers[_times];
        }
    
        println!("{}", result);
        break;
    }


}