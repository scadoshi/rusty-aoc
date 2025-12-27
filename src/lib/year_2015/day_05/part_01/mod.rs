mod niceometer;
use niceometer::Niceometer;

pub fn part_01(input: &[&'static [u8]]) -> usize {
    input
        .iter()
        .filter(|x| {
            x.iter()
                .fold(Niceometer::new(), |n, c| {
                    n.update_window(*c)
                        .update_has_double()
                        .update_vowel_count(*c)
                        .update_has_naughty()
                })
                .is_nice()
        })
        .count()
}
