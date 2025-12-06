use std::fs;
use day01::Day01;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            {
                let day01 = Day01::new(line_content.lines());
                println!("{}", day01.part1());
            }
            {
                let day01 = Day01::new(line_content.lines());
                println!("{}", day01.part2());
            }
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }

}
