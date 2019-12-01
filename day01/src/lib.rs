#![allow(dead_code)]

use std::fs;

fn read_input() -> Vec<u32> {
    fs::read_to_string("input/input.txt")
        .unwrap()
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect()
}

fn simulate(sheep_numbers: Vec<u32>) -> Option<u32> {
    let mut dragon_size = 50;
    let mut leftovers = 0;
    let mut days_gone_hungry = 0;

    for (day, num_sheep) in sheep_numbers.iter().enumerate() {
        let num_sheep = num_sheep + leftovers;

        if dragon_size <= num_sheep {
            leftovers = num_sheep - dragon_size;
            dragon_size += 1;
            days_gone_hungry = 0;
        } else {
            leftovers = 0;
            dragon_size -= 1;
            days_gone_hungry += 1;

            if days_gone_hungry >= 5 {
                return Some(day as u32);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let sheep_numbers = vec![50, 52, 52, 49, 50, 47, 45, 43, 50, 55];
        assert_eq!(simulate(sheep_numbers), Some(7));
    }

    #[test]
    fn test_solution() {
        let sheep_numbers = read_input();
        assert_eq!(simulate(sheep_numbers), Some(7602));
    }
}
