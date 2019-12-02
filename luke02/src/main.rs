use std::io::{self, Read};

fn water_count(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim().chars().filter(|&c| c == ' ').count())
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", water_count(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(29564, water_count(&input));
    }
}
