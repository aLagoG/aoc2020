use regex::Regex;
use std::collections::HashSet;

type InputType = Vec<HashSet<char>>;

pub fn input(input: &str) -> InputType {
    let re = Regex::new(r"\s").unwrap();
    input
        .split("\n\n")
        .map(|g| {
            let mut group = HashSet::new();
            for c in re.replace_all(g, "").chars() {
                group.insert(c);
            }
            group
        })
        .collect()
}

pub fn part1(input: InputType) -> usize {
    input.iter().fold(0, |a, g| a + g.len())
}

pub fn input2(input: &str) -> InputType {
    let mut base = HashSet::new();
    for c in 'a'..='z' {
        base.insert(c);
    }

    input
        .split("\n\n")
        .map(|g| {
            g.lines().fold(base.clone(), |a, l| {
                let mut group = HashSet::new();
                for c in l.trim().chars() {
                    group.insert(c);
                }
                a.intersection(&group).cloned().collect()
            })
        })
        .collect()
}

pub fn part2(input: InputType) -> usize {
    input.iter().fold(0, |a, g| a + g.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

        assert!(part1(input(input_str)) == 11);
    }

    #[test]
    fn p2() {
        let input_str = r"abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

        assert!(part2(input2(input_str)) == 6);
    }
}
