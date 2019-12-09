fn is_krampus(n: u64) -> bool {
    let s = (n * n).to_string();

    for i in 1..s.len() {
        let (a, b) = s.split_at(i);
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        if a != 0 && b != 0 && a + b == n {
            return true;
        }
    }

    false
}

fn main() {
    let sum = include_str!("../input/input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .filter(|&n| is_krampus(n))
        .sum::<u64>();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_krampus() {
        assert!(is_krampus(45));
        assert!(!is_krampus(100));
    }

    #[test]
    fn test_solution() {
        let sum = include_str!("../input/input.txt")
            .lines()
            .map(|l| l.parse().unwrap())
            .filter(|&n| is_krampus(n))
            .sum::<u64>();
        assert_eq!(445372, sum);
    }
}
