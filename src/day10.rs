use itertools::{Itertools, MinMaxResult};
use std::collections::HashSet;

type InputType = Vec<u32>;

pub fn input(input: &str) -> InputType {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

pub fn part1(mut input: InputType) -> u32 {
    input.sort();

    let mut b = vec![0];
    b.extend(&input);

    input.push(input.last().unwrap() + 3);

    let mut d1 = 0;
    let mut d3 = 0;
    for (l, h) in b.iter().zip(input) {
        match h - l {
            1 => d1 += 1,
            3 => d3 += 1,
            _ => {}
        }
    }

    d1 * d3
}

fn count_ways(idx: usize, input: &InputType, memo: &mut Vec<u64>, vst: &mut Vec<bool>) -> u64 {
    if vst[idx] {
        return memo[idx];
    }

    let mut res = 0;

    for i in 1..=3 {
        if idx + i >= input.len() {
            break;
        }
        if input[idx + i] <= input[idx] + 3 {
            res += count_ways(idx + i, input, memo, vst);
        }
    }

    res = res.max(1);

    vst[idx] = true;
    memo[idx] = res;
    res
}

pub fn part2(mut input: InputType) -> u64 {
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort();

    let mut memo = vec![0; input.len()];
    let mut vst = vec![false; input.len()];
    count_ways(0, &input, &mut memo, &mut vst);
    memo[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

        assert!(part1(input(input_str)) == 220);
    }
    #[test]
    fn p2() {
        let input_str = r"28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

        assert!(part2(input(input_str)) == 19208);
    }

    #[test]
    fn p2_small() {
        let input_str = r"16
10
15
5
1
11
7
19
6
12
4";

        assert!(part2(input(input_str)) == 8);
    }
}
