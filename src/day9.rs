use itertools::{Itertools, MinMaxResult};
use std::collections::HashSet;

type InputType = Vec<u64>;

pub fn input(input: &str) -> InputType {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

pub fn part1(input: InputType) -> u64 {
    #[cfg(not(test))]
    let window = 25;
    #[cfg(test)]
    let window = 5;

    let mut values = HashSet::with_capacity(window);
    for val in input.iter().take(window) {
        values.insert(*val);
    }

    for off in window..input.len() {
        let mut found = false;
        for check in values.iter() {
            if *check > input[off] {
                continue;
            }
            if values.contains(&(input[off] - check)) {
                found = true;
                break;
            }
        }
        if !found {
            return input[off];
        }
        values.remove(&input[off - window]);
        values.insert(input[off]);
    }

    0
}

pub fn part2(input: InputType) -> u64 {
    let goal = part1(input.clone());

    let mut sum = input[0];
    let mut start = 0;
    let mut end = 0;
    while end < input.len() && sum != goal {
        while sum < goal && end < input.len() {
            end += 1;
            sum += input[end];
        }

        while sum > goal {
            sum -= input[start];
            start += 1;
        }
    }

    let res = input[start..=end].iter().minmax();

    if let MinMaxResult::MinMax(min, max) = res {
        return min + max;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

        assert!(part1(input(input_str)) == 127);
    }

    #[test]
    fn p2() {
        let input_str = r"35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

        assert!(part2(input(input_str)) == 62);
    }
}
