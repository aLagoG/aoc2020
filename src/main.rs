use anyhow::Result;
use aoc_next::{aoc_main, failable_parser, parser, solution, solver, Aoc};

use aoc2020::*;

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2020,
    solutions: &[
        solution! {1, failable_parser!{ day1::input }, solver!{ day1::part1 }},
        solution! {1, failable_parser!{ day1::input }, solver!{ day1::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
