pub fn get_input() -> Vec<(u16, Vec<u16>, Vec<u16>)> {
    include_str!("input.txt")
        .lines()
        .filter_map(|l: &str| {
            let mut parts = l.split_whitespace();
            let bytes = parts.next()?.bytes().filter(|b| *b != b'[' && *b != b']');
            let len = bytes.clone().count();
            let lights = bytes.enumerate().fold(0u16, |mut acc, (i, b)| {
                if b == b'#'
                    && let Some(shift) = len
                        .checked_sub(1)
                        .and_then(|x| x.checked_sub(i))
                        .and_then(|x| u32::try_from(x).ok())
                    && let Some(incr) = 1u16.checked_shl(shift)
                {
                    acc += incr;
                }
                acc
            });
            let buttons: Vec<u16> = parts
                .clone()
                .filter(|x| x.starts_with('('))
                .map(|x| {
                    x.bytes()
                        .filter(|b| *b != b'(' && *b != b',' && *b != b')')
                        .fold(0u16, |mut acc, i| {
                            if let Some(num) = i.checked_sub(b'0')
                                && let Some(shift) = len
                                    .checked_sub(usize::from(num))
                                    .and_then(|x| x.checked_sub(1))
                                    .and_then(|x| u32::try_from(x).ok())
                                && let Some(incr) = 1u16.checked_shl(shift)
                            {
                                acc += incr;
                            }
                            acc
                        })
                })
                .collect();
            let joltage: Vec<u16> = parts
                .last()?
                .replace("{", "")
                .replace("}", "")
                .split(',')
                .filter_map(|x| x.trim().parse::<u16>().ok())
                .collect();
            Some((lights, buttons, joltage))
        })
        .collect()
}
