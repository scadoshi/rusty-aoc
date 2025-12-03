mod do_or_dont;
pub mod get_input;
mod match_and_mul;
pub mod part_01;
pub mod part_02;
pub use get_input::get_input;
pub use part_01::part_01;
pub use part_02::part_02;

use regex::Regex;
use std::sync::LazyLock;

static MUL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)$").unwrap());
const MUL_PATTERN_LEN: usize = 12;

static DO_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"do\(\)$").unwrap());
static DONT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"don\'t\(\)$").unwrap());
