use regex::Regex;

pub struct Day01<'a> {
    input: Vec<&'a str>
}

impl<'a> Day01<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day01{input: lines.collect()}
    }
    pub fn run(&self) -> i32 {
        let re = Regex::new(r"(L|R)(\d+)").unwrap();
        let mut sum_of_zeros = 0;
        let mut position = 50;
        for line in &self.input {
            for pairs in re.captures_iter(line).map(|caps| {
                let (_, [direction, count]) = caps.extract();
                (direction, count.parse::<i32>().unwrap())
            }) {
                if pairs.0 == "L" {
                    position -= pairs.1;
                } else {
                    position += pairs.1;
                }
                position %= 100;
                if position == 0 {
                    sum_of_zeros += 1;
                }
            }
        }
        sum_of_zeros
    }
}

#[cfg(test)]
mod tests {
    use crate::Day01;
    const SAMPLE: &str =
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn day01_sample_1() {
        let day01 = Day01::new(SAMPLE.lines());
        assert_eq!(3, day01.run());
    }
}