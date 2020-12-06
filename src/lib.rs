#![feature(str_split_once)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

use itertools::{iproduct, Itertools};
use regex::Regex;
use std::str::FromStr;

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

#[aoc_generator(day4)]
fn d4g(input: &str) -> Vec<Vec<(String, String)>> {
    input
        .split("\n\n")
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.split(":").map(&str::to_string).next_tuple())
                .flatten()
                .collect()
        })
        .collect()
}

fn get_word(input: &Vec<(String, String)>, word: &str) -> Option<String> {
    input
        .into_iter()
        .find(|(key, _)| key == &word)
        .map(|(_, val)| val.clone())
}

fn get_parsed<F: FromStr>(input: &Vec<(String, String)>, word: &str) -> Option<F> {
    get_word(input, word)?.parse().ok()
}

#[aoc(day4, part1)]
fn d4p1(input: &[Vec<(String, String)>]) -> usize {
    input
        .iter()
        .map(|pass| {
            get_word(pass, "byr")?;
            get_word(pass, "iyr")?;
            get_word(pass, "eyr")?;
            get_word(pass, "hgt")?;
            get_word(pass, "hcl")?;
            get_word(pass, "ecl")?;
            get_word(pass, "pid")?;
            Some(())
        })
        .flatten()
        .count()
}

#[aoc(day4, part2)]
fn d4p2(input: &[Vec<(String, String)>]) -> usize {
    input
        .iter()
        .filter(|passport| {
            if let (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) = (
                get_parsed::<u32>(passport, "byr"),
                get_parsed::<u32>(passport, "iyr"),
                get_parsed::<u32>(passport, "eyr"),
                get_word(passport, "hgt"),
                get_word(passport, "hcl"),
                get_word(passport, "ecl"),
                get_word(passport, "pid"),
            ) {
                [
                    (1920..=2002).contains(&byr),
                    (2010..=2020).contains(&iyr),
                    (2020..=2030).contains(&eyr),
                    Regex::new(r#"^#[\da-z]{6}$"#)
                        .unwrap()
                        .captures(&hcl)
                        .is_some(),
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()),
                    Regex::new(r#"^\d{9}$"#)
                        .unwrap()
                        .captures(&pid.as_str())
                        .is_some(),
                    (hgt.ends_with("cm")
                        && (150..=193)
                            .contains(&hgt.trim_end_matches("cm").parse::<u32>().unwrap_or(0)))
                        || (hgt.ends_with("in")
                            && (59..=76)
                                .contains(&hgt.trim_end_matches("in").parse::<u32>().unwrap_or(0))),
                ]
                .iter()
                .all(|&x| x)
            } else {
                false
            }
        })
        .count()
}

#[aoc_generator(day5)]
fn d5g(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| ['B', 'R'].contains(&character))
                .collect()
        })
        .collect()
}

#[aoc(day5, part1)]
fn d5p1(input: &[Vec<bool>]) -> u16 {
    input
        .iter()
        .map(|bool_vec| {
            bool_vec
                .iter()
                .fold(0, |number, &bit| (number << 1) + bit as u16)
        })
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
fn d5p2(input: &[Vec<bool>]) -> u16 {
    input
        .iter()
        .map(|bool_vec| {
            bool_vec
                .iter()
                .fold(0, |number, &bit| (number << 1) + bit as u16)
        })
        .sorted()
        .into_iter()
        .coalesce(|prev, curr| {
            if prev + 1 == curr {
                Ok(curr)
            } else {
                Err((prev, curr))
            }
        })
        .next()
        .unwrap_or(0)
        + 1
}

aoc_lib! {year = 2020}
