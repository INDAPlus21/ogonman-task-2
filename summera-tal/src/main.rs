use std::io;

fn main() {

        let mut nmr_times = String::new();
        io::stdin().read_line(&mut nmr_times).expect("Error");
        let nmr_times: usize = nmr_times.trim().parse().expect("Error");
        

        let mut numbers = String::new();
        io::stdin().read_line(&mut numbers).expect("Error");
        let mut numbers: Vec<u32> = numbers.trim().split(" ")
            .map(|component| component.parse::<u32>().ok().expect("Error"))
            .collect::<Vec<u32>>();

        numbers.sort();

        eprintln!("{}", nmr_times);

        let mut result: u32 = 0;
        if nmr_times%2 == 0{
            eprintln!("Even");
            for _index in (nmr_times/2)..nmr_times{
                result += numbers[_index];
            }
        }else{
            for _index in ((nmr_times+1)/2-1)..nmr_times{
                result += numbers[_index];
            }
        }
        println!("{}", result);

}