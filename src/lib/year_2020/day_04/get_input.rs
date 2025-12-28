pub fn get_input() -> Vec<&'static str> {
    include_str!("input.txt")
        .split("\n\n")
        //.map(|x| x.split_whitespace().collect())
        //.filter(|x: &Vec<&str>| !x.is_empty())
        .collect()
}
