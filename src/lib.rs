#![feature(str_split_once)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[aoc_generator(day1)]
fn d1g(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn _d1p1_loop(input: &[u64]) -> u64 {
    for (num1, num2) in input.into_iter().cartesian_product(input) {
        if num1 + num2 == 2020 {
            return num1 * num2;
        }
    }
    0
}

fn _d1p1_fold(input: &[u64]) -> u64 {
    input
        .into_iter()
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
        .into_iter()
        .cartesian_product(input)
        .find(|(&num1, &num2)| num1 + num2 == 2020)
        .map(|(num1, num2)| num1 * num2)
        .unwrap()
}

fn _d1p2_loop(input: &[u64]) -> u64 {
    for ((num1, num2), num3) in input
        .into_iter()
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
        .into_iter()
        .filter(|((low, high), character, password)| {
            (low..=high).contains(&&password.matches(*character).count())
        })
        .count()
        .clone()
}

#[aoc(day2, part2)]
fn d2p2(input: &[((usize, usize), char, String)]) -> usize {
    input
        .into_iter()
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
        .into_iter()
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
        .into_iter()
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
        .into_iter()
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
        .into_iter()
        .map(|bool_vec| {
            bool_vec
                .into_iter()
                .fold(0, |number, &bit| (number << 1) + bit as u16)
        })
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
fn d5p2(input: &[Vec<bool>]) -> u16 {
    input
        .into_iter()
        .map(|bool_vec| {
            bool_vec
                .into_iter()
                .fold(0, |number, &bit| (number << 1) + bit as u16)
        })
        .sorted()
        .tuple_windows()
        .find(|(current, next)| current + 1 != *next)
        .map(|(current, _)| current + 1)
        .unwrap_or(0)
}

#[aoc_generator(day6)]
fn d6g(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|person| {
                    person
                        .chars()
                        .fold(0, |set, character| set | char_to_mask(character))
                })
                .collect()
        })
        .collect()
}

fn char_to_mask(input: char) -> u32 {
    //finds the index n in the lowercase alphabet, and returns a 1-hot binary
    //number with the nth least significant bit set
    1 << (input as u32 - 'a' as u32)
}

#[aoc(day6, part1)]
fn d6p1(input: &[Vec<u32>]) -> u32 {
    input
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold(0, |set, person| person | set)
                .count_ones()
        })
        .sum()
}

#[aoc(day6, part2)]
fn d6p2(input: &[Vec<u32>]) -> u32 {
    input
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold(u32::MAX, |set, person| person & set)
                .count_ones()
        })
        .sum()
}

#[aoc_generator(day7)]
fn d7g(input: &str) -> Vec<(String, Vec<(u64, String)>)> {
    //light red bags contain 1 bright white bag, 2 muted yellow bags.
    input
        .lines()
        .map(|line| {
            let (key, value) = line.split(" bags contain ").next_tuple().unwrap();
            Some((
                key.to_string(),
                Regex::new(r"(\d+|no) (\w+ \w+) bags?(?:, |\.)")
                    .unwrap()
                    .captures_iter(value)
                    .map(|cap| (cap[1].parse().unwrap_or(1), cap[2].to_string()))
                    .collect(),
            ))
        })
        .flatten()
        .collect()
}

#[aoc(day7, part1)]
fn d7p1(_input: &[(String, Vec<(u64, String)>)]) -> String {
    todo!()
    /*
    let mut holdable = HashSet::with_capacity(input.len());
    //Vec<&str> = ["shiny gold"].into();
    let mut to_check = HashSet::with_capacity(input.len());//: Vec<&str> = input.into_iter().map(|(key, _)| key.as_str()).collect();
    while !to_check.is_empty() {
        let x = to_check.iter().next().unwrap();

    }
    todo!()*/
}

#[aoc(day7, part2)]
fn d7p2(_input: &[(String, Vec<(u64, String)>)]) -> String {
    todo!()
}

#[derive(Debug, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

type Instr = (Op, i32);

#[derive(Debug, Clone)]
struct VM {
    acc: i32,
    code: Vec<Instr>,
    ip: usize,
}

struct RunningVM(VM);

impl VM {
    fn with_code(code: Vec<Instr>) -> VM {
        VM {
            acc: 0,
            code,
            ip: 0,
        }
    }

    fn next_ip(&mut self) {
        self.ip += 1;
    }

    fn step_code(&mut self) -> bool {
        if self.ip >= self.code.len() {
            return false;
        }
        match self.code[self.ip] {
            (Op::Nop, _) => {
                self.next_ip();
            }
            (Op::Acc, value) => {
                self.acc += value;
                self.next_ip();
            }
            (Op::Jmp, value) => {
                self.ip += value as usize;
            }
        };
        true
    }
}

impl Iterator for RunningVM {
    type Item = VM;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.step_code() {
            Some(self.0.clone())
        } else {
            None
        }
    }
}

#[aoc_generator(day8)]
fn d8g(input: &str) -> Vec<Instr> {
    parse_asm(input)
}

fn parse_asm(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| line.split_whitespace().next_tuple().unwrap())
        .map(|(fst, snd)| (parse_op(fst).unwrap(), snd.parse().unwrap()))
        .collect()
}

fn parse_op(input: &str) -> Option<Op> {
    match input {
        "nop" => Some(Op::Nop),
        "acc" => Some(Op::Acc),
        "jmp" => Some(Op::Jmp),
        _ => None,
    }
}

#[aoc(day8, part1)]
fn d8p1(input: &[Instr]) -> i32 {
    let mut visited = HashSet::with_capacity(input.len());
    let vm = VM::with_code(input.to_vec());
    let mut previous = 0;
    for snapshot in RunningVM(vm) {
        if !visited.insert(snapshot.ip) {
            return previous;
        } else {
            previous = snapshot.acc;
        }
    }
    0
}

#[aoc(day8, part2)]
fn d8p2(input: &[Instr]) -> i32 {
    let mut index = 0;
    loop {
        let mut visited = HashSet::with_capacity(input.len());
        if index >= input.len() {
            panic!("Not found!")
        }
        match toggle_jmp_nop_command(input.clone().to_vec(), index) {
            Some(toggled) => {
                let mut last_snapshot_acc = None;
                let mut found = true;
                'inner: for snapshot in RunningVM(VM::with_code(toggled)) {
                    if !visited.insert(snapshot.ip) {
                        found = false;
                        break 'inner;
                    }
                    last_snapshot_acc = Some(snapshot.acc);
                }
                if found {
                    return last_snapshot_acc.unwrap();
                }
            }
            _ => {}
        }
        index += 1;
    }
}

fn toggle_jmp_nop_command(mut input: Vec<Instr>, index: usize) -> Option<Vec<Instr>> {
    match input[index] {
        (Op::Jmp, x) => {
            input[index] = (Op::Nop, x);
            Some(input)
        }
        (Op::Nop, x) => {
            input[index] = (Op::Jmp, x);
            Some(input)
        }
        _ => None,
    }
}

#[aoc_generator(day9)]
fn d9g(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse()).flatten().collect()
}

#[aoc(day9, part1)]
fn d9p1(input: &[u32]) -> u32 {
    input
        .windows(26)
        .find(|window| !is_pairwise_sum_from(&window[..25], window[25]))
        .unwrap()[25]
}

#[aoc(day9, part2)]
fn d9p2(input: &[u32]) -> u32 {
    let target = d9p1(input);
    let mut sums = input.iter().collect::<Vec<_>>();
    for offset in 1..=input.len() {
        sums = sums.iter().map(|&sum| Some(sum)).flatten().collect()
    }
    todo!()
}

fn is_pairwise_sum_from(population: &[u32], target: u32) -> bool {
    population
        .iter()
        .tuple_combinations()
        .find(|(&x, &y)| target == x + y)
        .is_some()
}

aoc_lib! {year = 2020}
