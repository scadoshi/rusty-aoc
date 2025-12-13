pub fn part_01(input: &[(u16, Vec<u16>, Vec<u16>)]) -> usize {
    input
        .iter()
        .filter_map(|line| {
            let (target, buttons, _joltage) = line;
            (0u16..(1 << buttons.len())).fold(None::<usize>, |fewest, subset| {
                if let Ok(current) = usize::try_from(subset.count_ones())
                    && fewest.is_none_or(|f| current < f)
                {
                    let state = (0..buttons.len()).fold(0u16, |state, i| {
                        if subset & 1 << i != 0
                            && let Some(button) = buttons.get(i)
                        {
                            state ^ button
                        } else {
                            state
                        }
                    });
                    if state == *target {
                        Some(current)
                    } else {
                        fewest
                    }
                } else {
                    fewest
                }
            })
        })
        .sum()
}
