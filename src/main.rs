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
        solution! {5, parser!{ day5::input }, solver!{ day5::part1 }},
        solution! {5, parser!{ day5::input }, solver!{ day5::part2 }},
        solution! {6, parser!{ day6::input }, solver!{ day6::part1 }},
        solution! {6, parser!{ day6::input2 }, solver!{ day6::part2 }},
        solution! {7, parser!{ day7::input }, solver!{ day7::part1 }},
        solution! {7, parser!{ day7::input }, solver!{ day7::part2 }},
        solution! {8, parser!{ day8::input }, solver!{ day8::part1 }},
        solution! {8, parser!{ day8::input }, solver!{ day8::part2 }},
        solution! {9, parser!{ day9::input }, solver!{ day9::part1 }},
        solution! {9, parser!{ day9::input }, solver!{ day9::part2 }},
        solution! {10, parser!{ day10::input }, solver!{ day10::part1 }},
        solution! {10, parser!{ day10::input }, solver!{ day10::part2 }},
        solution! {11, parser!{ day11::input }, solver!{ day11::part1 }},
        solution! {11, parser!{ day11::input }, solver!{ day11::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
