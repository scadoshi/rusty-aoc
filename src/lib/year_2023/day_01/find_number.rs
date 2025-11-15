use crate::year_2023::day_01::unspell_digit::UnspellDigit;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

#[allow(dead_code)]
static REGEX_NUMBERS: LazyLock<[Regex; 10]> = LazyLock::new(|| {
    [
        Regex::new("zero").expect("failed to create regex"),
        Regex::new("one").expect("failed to create regex"),
        Regex::new("two").expect("failed to create regex"),
        Regex::new("three").expect("failed to create regex"),
        Regex::new("four").expect("failed to create regex"),
        Regex::new("five").expect("failed to create regex"),
        Regex::new("six").expect("failed to create regex"),
        Regex::new("seven").expect("failed to create regex"),
        Regex::new("eight").expect("failed to create regex"),
        Regex::new("nine").expect("failed to create regex"),
    ]
});

#[allow(dead_code)]
static SREBMUN_XEGER: LazyLock<[Regex; 10]> = LazyLock::new(|| {
    [
        Regex::new("orez").expect("failed to create regex"),
        Regex::new("eno").expect("failed to create regex"),
        Regex::new("owt").expect("failed to create regex"),
        Regex::new("eerht").expect("failed to create regex"),
        Regex::new("ruof").expect("failed to create regex"),
        Regex::new("evif").expect("failed to create regex"),
        Regex::new("xis").expect("failed to create regex"),
        Regex::new("neves").expect("failed to create regex"),
        Regex::new("thgie").expect("failed to create regex"),
        Regex::new("enin").expect("failed to create regex"),
    ]
});

#[allow(dead_code)]
pub trait FindNumber {
    fn first_number(&self) -> Option<i32>;
    fn last_number(&self) -> Option<i32>;
    fn both_numbers(&self) -> Option<i32>;
}

#[allow(dead_code)]
impl FindNumber for &str {
    fn first_number(&self) -> Option<i32> {
        let spelled = REGEX_NUMBERS
            .iter()
            .filter_map(|r| r.find(self))
            .sorted_by(|a, b| a.start().cmp(&b.start()))
            .next();
        let numeric = self
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
            .next();
        match (spelled, numeric) {
            (Some(m), Some((i, n))) => {
                if m.start() < i {
                    m.as_str().unspell_digit()
                } else {
                    Some(n as i32)
                }
            }
            (None, Some((_, n))) => Some(n as i32),
            (Some(m), None) => m.as_str().unspell_digit(),
            (None, None) => None,
        }
    }

    fn last_number(&self) -> Option<i32> {
        let fles: String = self.chars().rev().collect();
        let spelled = SREBMUN_XEGER
            .iter()
            .filter_map(|r| r.find(&fles))
            .sorted_by(|a, b| a.start().cmp(&b.start()))
            .next();
        let numeric = fles
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
            .next();
        match (spelled, numeric) {
            (Some(m), Some((i, n))) => {
                if m.start() < i {
                    m.as_str().tigid_llepsnu()
                } else {
                    Some(n as i32)
                }
            }
            (None, Some((_, n))) => Some(n as i32),
            (Some(m), None) => m.as_str().tigid_llepsnu(),
            (None, None) => None,
        }
    }

    fn both_numbers(&self) -> Option<i32> {
        let Some(first) = self.first_number() else {
            return None;
        };
        let Some(last) = self.last_number() else {
            return None;
        };
        format!("{}{}", first, last).parse::<i32>().ok()
    }
}
