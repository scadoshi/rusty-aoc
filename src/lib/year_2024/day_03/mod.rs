pub mod do_or_dont;
pub mod get_input;
pub mod match_and_mul;
pub mod part_01;
pub mod part_02;

use regex::Regex;
use std::sync::LazyLock;

pub static MUL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)$").unwrap());
pub const MUL_PATTERN_LEN: usize = 12;

pub static DO_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"do\(\)$").unwrap());
pub static DONT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"don\'t\(\)$").unwrap());
