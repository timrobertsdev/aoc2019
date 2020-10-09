#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|mass| calculate_fuel_naive(mass))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|mass| calculate_fuel(mass))
        .sum()
}

fn calculate_fuel_naive(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn calculate_fuel(mass: &i32) -> i32 {
    let mut total_fuel = 0;
    let mut added_fuel = (mass / 3) - 2;

    while added_fuel > 0 {
        total_fuel += added_fuel;
        let new_fuel = (added_fuel / 3) - 2;
        added_fuel = new_fuel;
    }

    total_fuel
}
