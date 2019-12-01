use std::io::{self, Read};

struct Dragon {
    mass: u64,
    days_hungry: u64,
}

impl Dragon {
    fn new() -> Dragon {
        Dragon {
            mass: 50,
            days_hungry: 0,
        }
    }

    fn is_going_berserk(&self) -> bool {
        self.days_hungry >= 5
    }

    fn eat(&mut self, sheep: u64) -> u64 {
        if sheep < self.mass {
            self.mass -= 1;
            self.days_hungry += 1;
            0
        } else {
            let leftover = sheep - self.mass;
            self.mass += 1;
            self.days_hungry = 0;
            leftover
        }
    }
}

fn simulate(sheep_supplies: Vec<u64>) -> Option<u64> {
    let mut dragon = Dragon::new();
    let mut leftover = 0;

    for (day, &sheep) in sheep_supplies.iter().enumerate() {
        leftover = dragon.eat(sheep + leftover);
        if dragon.is_going_berserk() {
            return Some(day as u64);
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let sheep_supplies = input
        .split(',')
        .filter_map(|num| num.trim().parse().ok())
        .collect();

    println!("Luke 1: {}", simulate(sheep_supplies).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_simulate() {
        let sheep_supplies = vec![50, 52, 52, 49, 50, 47, 45, 43, 50, 55];
        assert_eq!(simulate(sheep_supplies), Some(7));
    }

    #[test]
    fn test_solution() {
        let sheep_supplies = fs::read_to_string("input/input.txt")
            .unwrap()
            .split(',')
            .filter_map(|num| num.trim().parse().ok())
            .collect();
        assert_eq!(simulate(sheep_supplies), Some(7602));
    }
}
