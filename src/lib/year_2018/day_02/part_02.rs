fn difference_count(string1: &str, string2: &str) -> usize {
    string1
        .chars()
        .zip(string2.chars())
        .filter(|(char1, char2)| char1 != char2)
        .count()
}

fn find_two_strings_with_single_char_difference(input: &[String]) -> Option<(String, String)> {
    for (i, s1) in input.iter().enumerate() {
        if let Some((_, s2)) = input
            .iter()
            .enumerate()
            .find(|(j, s2)| *j != i && difference_count(s1, s2) == 1)
        {
            return Some((s1.to_owned(), s2.to_owned()));
        }
    }
    None
}

fn common_chars(string1: &str, string2: &str) -> String {
    string1
        .chars()
        .zip(string2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}

#[allow(dead_code)]
pub fn part_02(input: &[String]) -> Option<String> {
    if let Some((s1, s2)) = find_two_strings_with_single_char_difference(input) {
        return Some(common_chars(&s1, &s2));
    }
    None
}
