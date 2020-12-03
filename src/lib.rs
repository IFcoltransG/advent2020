#![feature(str_split_once)]
extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day1)]
fn d1g(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn d1p1(input: &[usize]) -> usize {
    for (num1, num2) in input.iter().cartesian_product(input) {
        if num1 + num2 == 2020 {
            return num1 * num2;
        }
    }
    0
}

#[aoc(day1, part2)]
fn d1p2(input: &[usize]) -> usize {
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
                        cap.name("lower").unwrap().as_str().parse().unwrap(),
                        cap.name("upper").unwrap().as_str().parse().unwrap(),
                    ),
                    cap.name("character")
                        .unwrap()
                        .as_str()
                        .chars()
                        .next()
                        .unwrap(),
                    cap.name("password").unwrap().as_str().to_string(),
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
        .map(|line| line.chars().map(|chr| (chr == '#')).collect())
        .collect()
}

#[aoc(day3, part1)]
fn d3p1(input: &[Vec<bool>]) -> usize {
    let ratio = 3;
    let mut location = 0;
    let mut count = 0;
    for line in input {
        if line[location % line.len()] {
            count += 1
        }
        location += ratio
    }
    return count;
}

#[aoc(day3, part2)]
fn d3p2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(num, den)| check(input, *num, *den))
        .product()
}

fn check(input: &[Vec<bool>], num: usize, den: usize) -> usize {
    input
        .iter()
        .step_by(den)
        .enumerate()
        .filter(|(i, line)| line[i * num % line.len()])
        .count()
}

aoc_lib! {year = 2020}
