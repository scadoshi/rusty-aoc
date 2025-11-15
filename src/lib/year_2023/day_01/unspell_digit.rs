#[allow(dead_code)]
pub trait UnspellDigit {
    fn unspell_digit(&self) -> Option<i32>;
    fn tigid_llepsnu(&self) -> Option<i32>;
}

#[allow(dead_code)]
impl UnspellDigit for &str {
    fn unspell_digit(&self) -> Option<i32> {
        match *self {
            "zero" => Some(0),
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        }
    }

    fn tigid_llepsnu(&self) -> Option<i32> {
        self.chars()
            .rev()
            .collect::<String>()
            .as_str()
            .unspell_digit()
    }
}
