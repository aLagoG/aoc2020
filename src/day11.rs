#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Val {
    Empty,
    Full,
    NA,
}

impl From<char> for Val {
    fn from(v: char) -> Self {
        match v {
            '.' => Val::NA,
            'L' => Val::Empty,
            '#' => Val::Full,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Val::Empty => 'L',
                Val::Full => '#',
                Val::NA => '.',
            }
        )
    }
}

type InputType = Vec<Vec<Val>>;

macro_rules! add_idx {
    ($unsigned:expr, $signed:expr) => {
        if $signed < 0 {
            $unsigned - -$signed as usize
        } else {
            $unsigned + $signed as usize
        }
    };
}

pub fn input(input: &str) -> InputType {
    input
        .lines()
        .map(|l| l.trim().chars().map(|c| c.into()).collect())
        .collect()
}

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn simulate(
    input: InputType,
    count_fn: &dyn Fn(&InputType, usize, usize) -> i32,
    threshold: i32,
) -> u64 {
    let mut src = input.clone();
    let mut dest = input;

    let mut has_changes;

    loop {
        has_changes = false;
        for i in 0..src.len() {
            for j in 0..src[0].len() {
                if src[i][j] == Val::NA {
                    continue;
                }

                let count = count_fn(&src, i, j);

                match src[i][j] {
                    Val::Empty => {
                        if count == 0 {
                            dest[i][j] = Val::Full;
                            has_changes = true;
                        } else {
                            dest[i][j] = Val::Empty;
                        }
                    }
                    Val::Full => {
                        if count >= threshold {
                            dest[i][j] = Val::Empty;
                            has_changes = true;
                        } else {
                            dest[i][j] = Val::Full;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !has_changes {
            break;
        }
        std::mem::swap(&mut src, &mut dest);
    }

    dest.iter().fold(0, |a, v| {
        a + v.iter().fold(0, |a2, v2| match v2 {
            Val::Full => a2 + 1,
            _ => a2,
        })
    })
}

pub fn part1(input: InputType) -> u64 {
    let func = |src: &InputType, i, j| {
        let mut count = 0;
        for &(dx, dy) in DIRECTIONS.iter() {
            if dx.is_negative() && -dx as usize > i || dy.is_negative() && -dy as usize > j {
                continue;
            }

            let x = add_idx!(i, dx);
            let y = add_idx!(j, dy);
            if x >= src.len() || y >= src[0].len() {
                continue;
            }

            if src[x][y] == Val::Full {
                count += 1;
            }
        }
        count
    };

    simulate(input, &func, 4)
}

pub fn part2(input: InputType) -> u64 {
    let func = |src: &InputType, i, j| {
        let mut count = 0;
        for &(dx, dy) in DIRECTIONS.iter() {
            let mut seat = Val::NA;
            let mut x = i;
            let mut y = j;
            loop {
                if dx.is_negative() && -dx as usize > x || dy.is_negative() && -dy as usize > y {
                    break;
                }

                x = add_idx!(x, dx);
                y = add_idx!(y, dy);
                if x >= src.len() || y >= src[0].len() {
                    break;
                }

                match src[x][y] {
                    Val::NA => continue,
                    val => {
                        seat = val;
                        break;
                    }
                }
            }

            if seat == Val::Full {
                count += 1;
            }
        }
        count
    };

    simulate(input, &func, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

        assert!(part1(input(input_str)) == 37);
    }

    #[test]
    fn p2() {
        let input_str = r"L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

        assert!(part2(input(input_str)) == 26);
    }
}
