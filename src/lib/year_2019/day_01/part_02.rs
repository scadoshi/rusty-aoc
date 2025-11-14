use crate::year_2019::day_01::part_01::FuelRequired;

#[allow(dead_code)]
pub fn part_02(input: &[i32]) -> i32 {
    let mut needs_fuel: Vec<i32> = input.to_owned();
    let mut total = 0;
    while let Some(weight) = needs_fuel.pop() {
        let added = weight.fuel_required();
        if added > 0 {
            total += added;
            needs_fuel.push(added);
        }
    }

    total
}
