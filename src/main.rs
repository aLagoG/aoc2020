use anyhow::Result;
use aoc_next::{aoc_main, failable_parser, parser, solution, solver, Aoc};

use aoc2020::*;

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2020,
    solutions: &[
        solution! {1, failable_parser!{ day1::input }, solver!{ day1::part1 }},
        solution! {1, failable_parser!{ day1::input }, solver!{ day1::part2 }},
        solution! {2, parser!{ day2::input }, solver!{ day2::part1 }},
        solution! {2, parser!{ day2::input }, solver!{ day2::part2 }},
        solution! {3, parser!{ day3::input }, solver!{ day3::part1 }},
        solution! {3, parser!{ day3::input }, solver!{ day3::part2 }},
        solution! {4, parser!{ day4::input }, solver!{ day4::part1 }},
        solution! {4, parser!{ day4::input }, solver!{ day4::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
