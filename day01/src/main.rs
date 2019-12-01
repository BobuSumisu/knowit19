#![allow(dead_code)]

use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let sheep_numbers = input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let day = simulate(sheep_numbers).unwrap();
    println!("{}", day);

    Ok(())
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
    use std::fs;

    #[test]
    fn test_simulate() {
        let sheep_numbers = vec![50, 52, 52, 49, 50, 47, 45, 43, 50, 55];
        assert_eq!(simulate(sheep_numbers), Some(7));
    }

    #[test]
    fn test_solution() -> Result<()> {
        let sheep_numbers = fs::read_to_string("input/input.txt")?
            .split(',')
            .map(|v| v.trim().parse().unwrap())
            .collect();
        assert_eq!(simulate(sheep_numbers), Some(7602));
        Ok(())
    }
}
