
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let mut elf_calories: Vec<i32> = [].to_vec();
        let mut calories = 0;
        for line in lines {
            if let Ok(ln) = line {
                if ln == "" {
                    println!("{}", calories);
                    elf_calories.push(calories);
                    calories = 0;
                } else {
                    let cals: i32 = ln.parse().unwrap();
                    calories = calories + cals;
                }
            }
        
        }
        elf_calories.push(calories);

        elf_calories.sort();
        elf_calories.reverse();
        println!("the top three elves have calories: \n{}\n{}\n{}",
            elf_calories[0],
            elf_calories[1],
            elf_calories[2]
        );

        let tot_cals: i32 =  [elf_calories[0], elf_calories[1], elf_calories[2]].iter().sum();
        println!("total calories of the three elves with the most calories: {}",
            tot_cals
        )
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
