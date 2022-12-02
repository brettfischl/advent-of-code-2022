
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
        let mut most_calories = 0;
        let mut calories = 0;
        for line in lines {
            if let Ok(ln) = line {
                if ln == "" {
                    println!("{}", calories);
                    if calories > most_calories {
                        most_calories = calories;
                    }
                    calories = 0;
                } else {
                    let cals: i32 = ln.parse().unwrap();
                    calories = calories + cals;
                }
            }
        }

        println!("elf with the most calories has {} many calories", most_calories);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
