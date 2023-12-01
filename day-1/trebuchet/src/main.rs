use std::{convert::Infallible, iter::Sum, str::FromStr};

#[derive(Debug)]
struct Num(u32);

fn check_pattern(s: &str, idx: usize) -> Option<char> {
    let s = &s[idx..];
    if s.starts_with(P_ONE.0) || s.starts_with(P_ONE.1) {
        Some('1')
    } else if s.starts_with(P_TWO.0) || s.starts_with(P_TWO.1) {
        Some('2')
    } else if s.starts_with(P_THREE.0) || s.starts_with(P_THREE.1) {
        Some('3')
    } else if s.starts_with(P_FOUR.0) || s.starts_with(P_FOUR.1) {
        Some('4')
    } else if s.starts_with(P_FIVE.0) || s.starts_with(P_FIVE.1) {
        Some('5')
    } else if s.starts_with(P_SIX.0) || s.starts_with(P_SIX.1) {
        Some('6')
    } else if s.starts_with(P_SEVEN.0) || s.starts_with(P_SEVEN.1) {
        Some('7')
    } else if s.starts_with(P_EIGHT.0) || s.starts_with(P_EIGHT.1) {
        Some('8')
    } else if s.starts_with(P_NINE.0) || s.starts_with(P_NINE.1) {
        Some('9')
    } else {
        None
    }
}

const P_ONE: (&str, &str) = ("one", "1");
const P_TWO: (&str, &str) = ("two", "2");
const P_THREE: (&str, &str) = ("three", "3");
const P_FOUR: (&str, &str) = ("four", "4");
const P_FIVE: (&str, &str) = ("five", "5");
const P_SIX: (&str, &str) = ("six", "6");
const P_SEVEN: (&str, &str) = ("seven", "7");
const P_EIGHT: (&str, &str) = ("eight", "8");
const P_NINE: (&str, &str) = ("nine", "9");

impl FromStr for Num {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first = '0';
        let mut last = '0';
        let mut idx = 0;
        loop {
            if idx >= s.len() {
                break;
            }
            if let Some(ch) = check_pattern(s, idx) {
                first = ch;
                break;
            }
            idx += 1;
        }
        let mut idx = s.len() - 1;
        loop {
            if let Some(ch) = check_pattern(s, idx) {
                last = ch;
                break;
            }
            if idx == 0 {
                break;
            } else {
                idx -= 1;
            }
        }
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
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
