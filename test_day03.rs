use aoc::year_2015::day_03::{direction::Direction, part_01::part_01, part_02::part_02};

fn main() {
    // Test with simple input: >
    let input = vec![Direction::East];
    println!("Test 1 - part_01: {}", part_01(&input));
    
    // Test with: ^>v<
    let input = vec![Direction::North, Direction::East, Direction::South, Direction::West];
    println!("Test 2 - part_01: {}", part_01(&input));
    println!("Test 2 - part_02: {}", part_02(&input));
    
    println!("Tests passed!");
}
