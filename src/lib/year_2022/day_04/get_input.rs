pub type RangePair = ((u8, u8), (u8, u8));
pub fn get_input() -> Vec<RangePair> {
    include_str!("input.txt")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let (ls, le) = l.split_once('-').unwrap();
            let (ls, le) = (ls.parse::<u8>().unwrap(), le.parse::<u8>().unwrap());
            let (rs, re) = r.split_once('-').unwrap();
            let (rs, re) = (rs.parse::<u8>().unwrap(), re.parse::<u8>().unwrap());
            ((ls, le), (rs, re))
        })
        .collect()
}
