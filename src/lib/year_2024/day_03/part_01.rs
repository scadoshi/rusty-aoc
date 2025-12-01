use regex::Regex;

pub fn part_01(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            c[0].parse::<usize>()
                .expect(format!("cannot parse {:?} to usize", &c[0]).as_str())
                * c[1]
                    .parse::<usize>()
                    .expect(format!("cannot parse {:?} to usize", &c[1]).as_str())
        })
        .sum()
}

// c.iter().map(|v| v.parse::<usize>().unwrap()).product()
