use anyhow::{anyhow, Result};

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const KEYS: [&'static str; 8] = [
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
    "cid", // (Country ID)
];

pub struct Input {
    values: HashMap<String, String>,
}

type InputType = Vec<Input>;

pub fn input(input: &str) -> InputType {
    let re = Regex::new(r"(\w+):(\S+)").unwrap();
    input
        .split("\n\n")
        .map(|p| {
            let mut values = HashMap::new();
            for cap in re.captures_iter(p) {
                values.insert(cap[1].to_string(), cap[2].to_string());
            }
            Input { values }
        })
        .collect()
}

fn validate_key(key: &str, val: &str) -> Result<bool> {
    lazy_static! {
        static ref HGT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        static ref HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref ECL: HashSet<&'static str> = {
            let mut tmp = HashSet::new();
            for v in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter() {
                tmp.insert(*v);
            }
            tmp
        };
        static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    match key {
        "byr" => {
            let num: i32 = val.parse()?;
            Ok(val.len() == 4 && num >= 1920 && num <= 2002)
        }
        "iyr" => {
            let num: i32 = val.parse()?;
            Ok(val.len() == 4 && num >= 2010 && num <= 2020)
        }
        "eyr" => {
            let num: i32 = val.parse()?;
            Ok(val.len() == 4 && num >= 2020 && num <= 2030)
        }
        "hgt" => {
            let cap = HGT
                .captures_iter(val)
                .next()
                .ok_or(anyhow!("No capture on hgt"))?;
            let num: i32 = cap[1].parse()?;
            let unit = &cap[2];
            match unit {
                "cm" => Ok(num >= 150 && num <= 193),
                "in" => Ok(num >= 59 && num <= 76),
                _ => Ok(false),
            }
        }
        "hcl" => Ok(HCL.is_match(val)),
        "ecl" => Ok(ECL.contains(val)),
        "pid" => Ok(PID.is_match(val)),
        "cid" => Ok(true),
        _ => Ok(false),
    }
}

fn is_valid(input: &Input) -> bool {
    for k in KEYS.iter() {
        if *k != "cid" && !input.values.contains_key(*k) {
            return false;
        }
    }

    true
}

pub fn part1(input: InputType) -> i32 {
    input
        .iter()
        .fold(0, |a, i| if is_valid(i) { a + 1 } else { a })
}

fn is_valid_p2(input: &Input) -> bool {
    for k in KEYS.iter().filter(|k| **k != "cid") {
        if !input.values.contains_key(*k) || !validate_key(*k, &input.values[*k]).unwrap_or(false) {
            return false;
        }
    }

    true
}

pub fn part2(input: InputType) -> i32 {
    input
        .iter()
        .fold(0, |a, i| if is_valid_p2(i) { a + 1 } else { a })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        assert!(part1(input(input_str)) == 2);
    }

    #[test]
    fn p2_invalid() {
        let input_str = r"eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        assert!(!input(input_str).iter().any(|p| is_valid_p2(p)));
    }

    #[test]
    fn p2_valid() {
        let input_str = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert!(input(input_str).iter().all(|p| is_valid_p2(p)));
    }
}
