use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    println!("Hello, world!");
}

fn solve_part_1() -> u64 {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum : u64 = 0;
    
    for line in reader.lines() {
        let value = line.unwrap();
        sum += fuel_required(value.parse::<u64>().unwrap());
    }
    return sum;
}

fn solve_part_2() -> i64 {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum : i64 = 0;
    
    for line in reader.lines() {
        let value = line.unwrap();
        sum += fuel_required_2(value.parse::<i64>().unwrap());
    }
    return sum;
}

fn fuel_required(mass: u64) -> u64 {
    return (mass / 3) - 2;
}

fn fuel_required_2(mass: i64) -> i64 {
    let fuel : i64 = (mass / 3) - 2;

    if(fuel > 0) {
        return fuel + fuel_required_2(fuel);
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_fuel_required_2() {
        assert_eq!(fuel_required_2(14), 2);
        assert_eq!(fuel_required_2(1969), 966);
        assert_eq!(fuel_required_2(100756), 50346);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(), 3386686);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(), 5077155);
    }
}
