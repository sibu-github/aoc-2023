use std::{convert::Infallible, iter::Sum, str::FromStr};

#[derive(Debug)]
struct Num(u32);

fn check_pattern(s: &str, idx: usize) -> Option<u32> {
    let s = &s[idx..];
    ALL_PATTERNS
        .iter()
        .find(|p| s.starts_with(p.0) || s.starts_with(p.1))
        .map(|p| p.2)
}

type PATTERN<'a> = (&'a str, &'a str, u32);
const P_ONE: PATTERN = ("one", "1", 1);
const P_TWO: PATTERN = ("two", "2", 2);
const P_THREE: PATTERN = ("three", "3", 3);
const P_FOUR: PATTERN = ("four", "4", 4);
const P_FIVE: PATTERN = ("five", "5", 5);
const P_SIX: PATTERN = ("six", "6", 6);
const P_SEVEN: PATTERN = ("seven", "7", 7);
const P_EIGHT: PATTERN = ("eight", "8", 8);
const P_NINE: PATTERN = ("nine", "9", 9);
const ALL_PATTERNS: [PATTERN; 9] = [
    P_ONE, P_TWO, P_THREE, P_FOUR, P_FIVE, P_SIX, P_SEVEN, P_EIGHT, P_NINE,
];

impl FromStr for Num {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first = 0;
        let mut last = 0;
        for idx in 0..s.len() {
            if let Some(n) = check_pattern(s, idx) {
                first = n;
                break;
            }
        }
        for idx in (0..s.len()).rev() {
            if let Some(n) = check_pattern(s, idx) {
                last = n;
                break;
            }
        }
        let num = first * 10 + last;
        Ok(Self(num))
    }
}

impl<'a> Sum<&'a Num> for u32 {
    fn sum<I: Iterator<Item = &'a Num>>(iter: I) -> Self {
        iter.map(|i| i.0).sum()
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let nums = content
        .lines()
        .map(|line| Num::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let sum = nums.iter().sum::<u32>();
    println!("{sum}");
}
