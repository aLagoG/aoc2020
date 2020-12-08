use anyhow::{anyhow, Result};
use std::collections::HashSet;

type InputType = Vec<i32>;

pub fn input(input: &str) -> Result<InputType> {
    input
        .lines()
        .map(|l| {
            l.parse()
                .map_err(|e| anyhow!("Failed to parse '{}': {:?}", l, e))
        })
        .collect()
}

pub fn part1(input: InputType) -> i32 {
    let mut set = HashSet::with_capacity(input.len());
    for v in input.iter() {
        set.insert(*v);
    }

    for v in input {
        let val = 2020 - v;
        if set.contains(&val) {
            return v * val;
        }
    }

    -1
}

pub fn part2(input: InputType) -> i32 {
    let mut set = HashSet::with_capacity(input.len());
    for v in input.iter() {
        set.insert(*v);
    }

    for i in 0..input.len() {
        let v = input[i];
        for j in (i + 1)..input.len() {
            let v2 = input[j];
            if v + v2 >= 2020 {
                continue;
            }
            let val = 2020 - v - v2;
            if set.contains(&val) {
                return v * val * v2;
            }
        }
    }

    -1
}
