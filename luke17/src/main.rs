struct TriangularNumberIter {
    n: u64,
    i: u64,
}

fn triangular_numbers() -> TriangularNumberIter {
    TriangularNumberIter { n: 0, i: 0 }
}

impl Iterator for TriangularNumberIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += self.i;
        self.i += 1;
        Some(self.n)
    }
}

struct RotationIter {
    n: u64,
    num_digits: usize,
    i: usize,
    mul_val: u64,
}

fn rotations_a(n: u64) -> RotationIter {
    let num_digits = num_digits(n);
    RotationIter {
        n,
        num_digits,
        i: 0,
        mul_val: 10.0_f64.powi((num_digits - 1) as i32) as u64,
    }
}

impl Iterator for RotationIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.num_digits {
            let r = Some(self.n);
            self.n = (self.n / 10) + (self.mul_val * (self.n % 10));
            self.i += 1;
            r
        } else {
            None
        }
    }
}

fn num_digits(n: u64) -> usize {
    if n < 10 {
        return 1;
    }

    let mut n = n;
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn is_square(n: u64) -> bool {
    let root = (n as f64).sqrt();
    (root - root.floor()) < std::f64::EPSILON
}

fn solution(n: usize) -> usize {
    triangular_numbers()
        .take(n)
        .filter(|&n| rotations_a(n).any(|x| is_square(x)))
        .count()
}

fn main() {
    println!("{}", solution(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(1_000_000), 74);
    }
}
