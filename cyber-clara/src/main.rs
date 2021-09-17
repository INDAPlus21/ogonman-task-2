    use std::io;
    use std::io::prelude::*;

    fn main() {

        let input = io::stdin();
        let mut lines = input
            .lock()
            .lines()
            .map(|_line| _line.ok().unwrap());

        let nmr_people:usize = lines 
        .next().unwrap()                                      
        .parse::<usize>().unwrap();     
        
        let mut first_name = Vec::<String>::new();

        let mut last_name = Vec::<String>::new();

        for _case in 0..(nmr_people) {
            first_name.push(lines.next().unwrap());
        }
        for _case in 0..(nmr_people) {
            last_name.push(lines.next().unwrap());
        }

        let mut people:Vec<String> = first_name.iter()
        .zip(last_name.iter())
        .map(|(f, l)| format!("{} {}", f, l))
        .collect();

        people.sort();
        people.dedup();

        println!("{}", (people.len()));

    }