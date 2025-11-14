pub trait FuelRequired {
    fn fuel_required(&self) -> i32;
}

impl FuelRequired for i32 {
    fn fuel_required(&self) -> i32 {
        ((self / 3) - 2).max(0)
    }
}

#[allow(dead_code)]
pub fn part_01(input: &[i32]) -> i32 {
    input.iter().map(|x| x.fuel_required()).sum()
}
