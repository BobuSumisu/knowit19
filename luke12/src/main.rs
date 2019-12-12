const MAGIC_NUMBER: i64 = 6174;

fn to_digits(n: i64) -> Vec<i64> {
    let mut digits = vec![];
    let mut n = n;

    while n != 0 {
        digits.insert(0, n % 10);
        n /= 10;
    }

    // We want leading zeroes.
    while digits.len() != 4 {
        digits.insert(0, 0);
    }

    digits
}

fn from_digits(digits: &[i64]) -> i64 {
    let mut n = 0;

    for d in digits {
        n = (n * 10) + d;
    }

    n
}

fn rising(n: i64) -> i64 {
    let mut digits = to_digits(n);
    digits.sort();
    from_digits(&digits)
}

fn falling(n: i64) -> i64 {
    let mut digits = to_digits(n);
    digits.sort();
    digits.reverse();
    from_digits(&digits)
}

fn find_magic(n: i64) -> usize {
    let mut n = n;
    let mut i = 0;

    while n != MAGIC_NUMBER && n != 0 {
        let x = rising(n);
        let y = falling(n);
        n = if x < y { y - x } else { x - y };
        i += 1;
    }

    i
}

fn count_sevens() -> usize {
    let mut count = 0;
    for i in 1000..9999 {
        if i % 1111 == 0 {
            continue;
        }
        if find_magic(i) == 7 {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_sevens());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(count_sevens(), 1980);
    }
}
