#![feature(str_split_once)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

use itertools::{iproduct, Itertools};
use regex::Regex;

#[aoc_generator(day1)]
fn d1g(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn _d1p1_loop(input: &[u64]) -> u64 {
    for (num1, num2) in input.iter().cartesian_product(input) {
        if num1 + num2 == 2020 {
            return num1 * num2;
        }
    }
    0
}

fn _d1p1_fold(input: &[u64]) -> u64 {
    input
        .iter()
        .cartesian_product(input)
        .fold(None, |acc, (num1, num2)| {
            if num1 + num2 == 2020 {
                Some(num1 * num2)
            } else {
                acc
            }
        })
        .unwrap()
}

#[aoc(day1, part1)]
fn d1p1_find(input: &[u64]) -> u64 {
    input
        .iter()
        .cartesian_product(input)
        .find(|(&num1, &num2)| num1 + num2 == 2020)
        .map(|(num1, num2)| num1 * num2)
        .unwrap()
}

fn _d1p2_loop(input: &[u64]) -> u64 {
    for ((num1, num2), num3) in input
        .iter()
        .cartesian_product(input)
        .cartesian_product(input)
    {
        if num1 + num2 + num3 == 2020 {
            return num1 * num2 * num3;
        }
    }
    0
}

#[aoc(day1, part2)]
fn d1p2_find(input: &[u64]) -> u64 {
    iproduct!(input, input, input)
        .find(|(&num1, &num2, &num3)| num1 + num2 + num3 == 2020)
        .map(|(num1, num2, num3)| num1 * num2 * num3)
        .unwrap()
}

fn _d2g_splits(input: &str) -> Vec<((usize, usize), char, String)> {
    input
        .lines()
        .map(|line| {
            //"3-4 j: hjvj"
            let (lowerbound, rest) = line.trim().split_once('-').unwrap();
            let (upperbound, rest) = rest.split_once(' ').unwrap();
            let (character, password) = rest.split_once(": ").unwrap();
            (
                (lowerbound.parse().unwrap(), upperbound.parse().unwrap()),
                character.chars().next().unwrap(),
                password.to_string(),
            )
        })
        .collect()
}

#[aoc_generator(day2)]
fn d2g_regex(input: &str) -> Vec<((usize, usize), char, String)> {
    //e.g. "3-4 j: hjvj"
    let re =
        Regex::new(r#"^(?P<lower>\d+)-(?P<upper>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)$"#)
            .unwrap();
    input
        .lines()
        .map(|line| {
            re.captures(line).and_then(|cap| {
                Some((
                    (
                        cap.name("lower")?.as_str().parse().ok()?,
                        cap.name("upper")?.as_str().parse().ok()?,
                    ),
                    cap.name("character")?.as_str().chars().next()?,
                    cap.name("password")?.as_str().to_string(),
                ))
            })
        })
        .flatten()
        .collect()
}

#[aoc(day2, part1)]
fn d2p1(input: &[((usize, usize), char, String)]) -> usize {
    input
        .iter()
        .filter(|((low, high), character, password)| {
            (low..=high).contains(&&password.matches(*character).count())
        })
        .count()
        .clone()
}

#[aoc(day2, part2)]
fn d2p2(input: &[((usize, usize), char, String)]) -> usize {
    input
        .iter()
        .filter(|((first, second), character, password)| {
            (password.chars().nth(*first - 1).unwrap() == *character)
                != (password.chars().nth(*second - 1).unwrap() == *character)
        })
        .count()
        .clone()
}

#[aoc_generator(day3)]
fn d3g(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|chr| chr == '#').collect())
        .collect()
}

#[aoc(day3, part1)]
fn d3p1(input: &[Vec<bool>]) -> usize {
    check_ratio_d3(input, 3, 1)
}

#[aoc(day3, part2)]
fn d3p2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(num, den)| check_ratio_d3(input, *num, *den))
        .product()
}

fn check_ratio_d3(input: &[Vec<bool>], num: usize, den: usize) -> usize {
    input
        .iter()
        .step_by(den)
        .enumerate()
        .filter(|(i, line)| line[i * num % line.len()])
        .count()
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[aoc_generator(day4)]
fn d4g(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|line| {
            let values = line.split_whitespace().map(|entry| {
                //println!("{}", entry);
                let mut values = entry.split(":").into_iter();
                (
                    values.next().unwrap().to_string(),
                    values.next().unwrap().to_string(),
                )
            });
            get_passport(values.collect())
        })
        .collect()
}

fn get_passport(input: Vec<(String, String)>) -> Passport {
    Passport {
        byr: get_word(&input, "byr".to_string()),
        iyr: get_word(&input, "iyr".to_string()),
        eyr: get_word(&input, "eyr".to_string()),
        hgt: get_word(&input, "hgt".to_string()),
        hcl: get_word(&input, "hcl".to_string()),
        ecl: get_word(&input, "ecl".to_string()),
        pid: get_word(&input, "pid".to_string()),
        cid: get_word(&input, "cid".to_string()),
    }
}

fn get_word(input: &Vec<(String, String)>, word: String) -> Option<String> {
    input
        .into_iter()
        .find(|(key, _)| key == &word)
        .map(|(_, val)| val.clone())
}

#[aoc(day4, part1)]
fn d4p1(input: &[Passport]) -> usize {
    input
        .iter()
        .map(|pass| {
            pass.byr.as_ref()?;
            pass.iyr.as_ref()?;
            pass.eyr.as_ref()?;
            pass.hgt.as_ref()?;
            pass.hcl.as_ref()?;
            pass.ecl.as_ref()?;
            pass.pid.as_ref()?;
            //pass.cid?;
            Some(())
        })
        .flatten()
        .count()
}

#[aoc(day4, part2)]
fn d4p2(input: &[Passport]) -> usize {
    input
        .iter()
        .map(|pass| {
            if (1920..=2002).contains(&pass.byr.as_ref()?.parse::<i32>().ok()?) &&
            (2010..=2020).contains(&pass.iyr.as_ref()?.parse::<i32>().ok()?) &&
            (2020..=2030).contains(&pass.eyr.as_ref()?.parse::<i32>().ok()?) {} else {return None}
            let hgt = pass.hgt.as_ref()?;
            if (hgt.ends_with("cm")
                && (150..=193).contains(&hgt.trim_end_matches("cm").parse::<i32>().ok()?))
                || (hgt.ends_with("in")
                    && (59..=76).contains(&hgt.trim_end_matches("in").parse::<i32>().ok()?))
            {
            } else {
                return None;
            }
            let (mut hcl1, hcl2) = pass.hcl.as_ref()?.chars().tee();
            if hcl1.next()? != '#' {
                return None;
            }
            let (hcl3, mut hcl4) = hcl2.skip(1).tee();
            if hcl3.count() != 6 {
                return None;
            }
            if hcl4.any(|character| !"0123456789abcdef".contains(character)) {
                return None;
            }
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&pass.ecl.as_ref()?) {
                return None;
            };
            let pid = &pass.pid.as_ref()?;
            if !pid.chars().all(|character| character.is_alphanumeric()) || pid.len() != 9 {
                return None;
            }
            //pass.cid?;
            Some(())
        })
        .flatten()
        .count()
}

aoc_lib! {year = 2020}
