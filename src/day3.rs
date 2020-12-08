use std::ops::Index;

pub struct Input {
    inner: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Input {
    pub fn new(inner: Vec<Vec<bool>>) -> Self {
        let width = inner[0].len();
        let height = inner.len();
        Self {
            inner,
            width,
            height,
        }
    }
}

impl Index<(usize, usize)> for Input {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[index.0 % self.height][index.1 % self.width]
    }
}

type InputType = Input;

pub fn input(input: &str) -> InputType {
    Input::new(
        input
            .lines()
            .map(|l| l.trim().chars().map(|c| c == '#').collect())
            .collect(),
    )
}

pub fn part1(input: InputType) -> i32 {
    let mut count = 0;

    let mut i = 1;
    let mut j = 3;

    while i < input.height {
        if input[(i, j)] {
            count += 1
        }
        i += 1;
        j += 3;
    }

    count
}

pub fn part2(input: InputType) -> i64 {
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let mut counts = Vec::new();

    for (di, dj) in slopes {
        let mut i = di;
        let mut j = dj;

        let mut count = 0;
        while i < input.height {
            if input[(i, j)] {
                count += 1
            }
            i += di;
            j += dj;
        }

        counts.push(count);
    }

    counts.iter().fold(1, |a, v| a * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        assert!(part1(input(input_str)) == 7);
    }

    #[test]
    fn p2() {
        let input_str = r"..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        assert!(part2(input(input_str)) == 336);
    }
}
