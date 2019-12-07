fn main() {
    println!("{}", special_div(5897, 7));
}

const N: u64 = 27_644_437;

fn special_div(x: u64, y: u64) -> u64 {
    let mut z = 0;

    for z_ in 2..N {
        if (z_ * y) % N == 1 {
            z = z_;
            break;
        }
    }

    (x * z) % N
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(13825167, special_div(5897, 2));
        assert_eq!(9216778, special_div(5897, 3));
        assert_eq!(20734802, special_div(5897, 4));
        assert_eq!(7899253, special_div(5897, 7));
    }
}
