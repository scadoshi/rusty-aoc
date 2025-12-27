mod niceometer;
use niceometer::Niceometer;

pub fn part_02(input: &[&'static [u8]]) -> usize {
    input
        .iter()
        .filter(|x| {
            x.iter()
                .fold(Niceometer::new(), |n, c| {
                    n.update_window(*c).update_pairs().update_has_aba()
                })
                .is_nice()
        })
        .count()
}
