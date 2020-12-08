use itertools::Itertools;

pub enum Direction {
    Min,
    Max,
}

type InputType = Vec<Vec<Direction>>;

pub fn input(input: &str) -> InputType {
    input
        .lines()
        .map(|l| {
            let mut res = l
                .trim()
                .chars()
                .take(7)
                .map(|c| match c {
                    'F' => Direction::Min,
                    'B' => Direction::Max,
                    _ => panic!(),
                })
                .collect_vec();
            res.extend(l.trim().chars().skip(7).take(3).map(|c| match c {
                'L' => Direction::Min,
                'R' => Direction::Max,
                _ => panic!(),
            }));
            res
        })
        .collect()
}

#[derive(PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

struct BinSearch {
    min: usize,
    max: usize,
}

impl BinSearch {
    pub fn search(&mut self, input: &[Direction]) -> usize {
        for d in input.iter() {
            match d {
                Direction::Min => {
                    self.max = self.min + ((self.max - self.min) / 2);
                }
                Direction::Max => {
                    self.min = self.min + ((self.max - self.min) as f32 / 2.0).ceil() as usize;
                }
            }
        }
        assert!(self.min == self.max);

        self.min
    }
}

fn position(input: &[Direction]) -> Pos {
    let mut bs = BinSearch { min: 0, max: 127 };
    let row = bs.search(&input[0..7]);

    bs.min = 0;
    bs.max = 7;
    let col = bs.search(&input[7..]);

    Pos { row, col }
}

pub fn part1(input: InputType) -> usize {
    let mut max = 0;

    for i in input {
        let pos = position(&i);
        let res = pos.row * 8 + pos.col;

        max = max.max(res);
    }

    max
}

pub fn part2(input: InputType) -> usize {
    let mut ids = Vec::with_capacity(input.len());

    for i in input {
        let pos = position(&i);
        ids.push(pos.row * 8 + pos.col);
    }

    ids.sort_unstable();

    for i in 1..ids.len() {
        if ids[i] - ids[i - 1] > 1 {
            return ids[i] - 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"FBFBBFFRLR
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL";

        let vals = input(input_str);

        assert!(position(&vals[0]) == Pos { row: 44, col: 5 });
        assert!(position(&vals[1]) == Pos { row: 70, col: 7 });
        assert!(position(&vals[2]) == Pos { row: 14, col: 7 });
        assert!(position(&vals[3]) == Pos { row: 102, col: 4 });
    }
}
