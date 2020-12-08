use itertools::Itertools;
use regex::Regex;

pub struct Input {
    pub min: i32,
    pub max: i32,
    pub char: char,
    pub pass: String,
}

type InputType = Vec<Input>;

pub fn input(input: &str) -> InputType {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input
        .lines()
        .map(|l| {
            let cap = re.captures_iter(l).next().unwrap();
            let min = cap[1].parse().unwrap();
            let max = cap[2].parse().unwrap();
            let char = cap[3].chars().next().unwrap();
            let pass = cap[4].to_string();
            Input {
                min,
                max,
                char,
                pass,
            }
        })
        .collect()
}

pub fn part1(input: InputType) -> i32 {
    let mut count = 0;
    for v in input {
        let val = v
            .pass
            .chars()
            .fold(0, |a, c| if c == v.char { a + 1 } else { a });
        if val >= v.min && val <= v.max {
            count += 1;
        }
    }

    count
}

pub fn part2(input: InputType) -> i32 {
    let mut count = 0;
    for v in input {
        let chars = v.pass.chars().collect_vec();
        if (chars[(v.min - 1) as usize] == v.char) != (chars[(v.max - 1) as usize] == v.char) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";

        assert!(part1(input(input_str)) == 2);
    }

    #[test]
    fn p2() {
        let input_str = r"1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";

        assert!(part2(input(input_str)) == 1);
    }
}
