use regex::Regex;

pub struct Day01<'a> {
    input: Vec<&'a str>
}

impl<'a> Day01<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day01{input: lines.collect()}
    }
    pub fn part1(&self) -> i32 {
        let mut sum_of_zeros = 0;
        let mut position = 50;
        self.run(|shift_amount| {
            position += shift_amount;
            position = position.rem_euclid(100);
            if position == 0 {
                sum_of_zeros += 1;
            }
        });
        sum_of_zeros
    }
    pub fn part2(&self) -> i32 {
        let mut sum_of_zeros = 0;
        let mut position = 50;
        self.run(|shift_amount| {
            let mut count_of_zeros = 0;
            let mut shift_amount = shift_amount;
            if shift_amount.abs() > 100 {
                count_of_zeros += shift_amount.abs() / 100;
                shift_amount %= 100;
            }
            if shift_amount < 0 && position == 0 {
                position = 100; // we want to be able to subtract the shift amount but not count as passing 0
            }
            position += shift_amount;
            if position == 0 {
                count_of_zeros += 1;
            } else if position < 0 {
                position += 100;
                count_of_zeros += 1;
            } else if position > 99 {
                position -= 100;
                count_of_zeros += 1;
            }
            assert!(position >= 0 && position < 100);
            sum_of_zeros += count_of_zeros;

        });
        sum_of_zeros
    }
    pub fn run<F>(&self, mut shift_dial_by: F) where F: FnMut(i32) {
        let re = Regex::new(r"(L|R)(\d+)").unwrap();

        for line in &self.input {
            for pairs in re.captures_iter(line).map(|caps| {
                let (_, [direction, count]) = caps.extract();
                (direction, count.parse::<i32>().unwrap())
            }) {
                let amount = if pairs.0 == "L" { -pairs.1 } else { pairs.1 };
                shift_dial_by(amount);
            }
        }
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
        assert_eq!(3, day01.part1());
    }

    #[test]
    fn day01p2_sample_1() {
        let day01 = Day01::new(SAMPLE.lines());
        assert_eq!(6, day01.part2());
    }

    #[test]
    fn day01p2_sample_2_right() {
        let day01 = Day01::new("R1000".lines());
        assert_eq!(10, day01.part2());
    }

    #[test]
    fn day01p2_sample_2_left() {
        let day01 = Day01::new("L1000".lines());
        assert_eq!(10, day01.part2());
    }

    #[test]
    fn test_mod_neg_100() {
        assert_eq!(0, -100 % 100);
        assert_eq!(1, ((-100 / 100) as i32).abs());
    }

    #[test]
    fn test_shift_negative_from_0_doesnt_count() {
        let day01 = Day01::new("L50\nL50".lines());
        assert_eq!(1, day01.part2());
    }

    #[test]
    fn test_shift_negative_from_0_landing_on_0_counts() {
        let day01 = Day01::new("L50\nL100".lines());
        assert_eq!(2, day01.part2());
    }

    #[test]
    fn test_shift_positive_from_0_landing_on_100_counts() {
        let day01 = Day01::new("R50\nR100".lines());
        assert_eq!(2, day01.part2());
    }

    #[test]
    fn test_left_shift_to_land_on_multiple_of_100_counts() {
        let day01 = Day01::new("L150".lines());
        assert_eq!(2, day01.part2());
    }
}
