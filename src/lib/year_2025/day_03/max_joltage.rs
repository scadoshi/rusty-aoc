use std::usize;

pub type Bank = Vec<u8>;
pub trait MaxJoltage {
    fn max_joltage(&self, count: u8) -> usize;
}
impl MaxJoltage for Bank {
    fn max_joltage(&self, count: u8) -> usize {
        if usize::from(count) > self.len() {
            panic!("oops");
        }
        (0..count)
            .rev()
            .fold((0, 0), |(acc, start), pow| {
                let end = self.len() - usize::from(pow) - 1;
                let (index, max) = self
                    .iter()
                    .enumerate()
                    .skip(start)
                    .take_while(|(i, _)| *i <= end)
                    .max_by(|(i, a), (j, b)| match a.cmp(b) {
                        std::cmp::Ordering::Equal => j.cmp(i),
                        _ => a.cmp(b),
                    })
                    .unwrap();
                (
                    acc + usize::from(*max) * 10_usize.pow(u32::from(pow)),
                    index + 1,
                )
            })
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_voltage() {
        let input: Vec<Vec<u8>> =
            "987654321111111\n811111111111119\n234234234234278\n818181911112111"
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect();
        let result: usize = input.iter().map(|x| x.max_joltage(2)).sum();
        assert_eq!(result, 357);
    }
}
