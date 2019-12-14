fn solution() -> u64 {
    let alphabet = &[2, 3, 5, 7, 11];
    let target = 217_532_235;

    let mut sequence = vec![0; target];
    let mut sequence_len = 0;

    let mut index = 0;
    let mut sum = 0;

    'outer: loop {
        for &digit in alphabet {
            let val = if index == 0 { digit } else { sequence[index] };

            for _ in 0..val {
                if digit == 7 {
                    sum += 7;
                }

                sequence[sequence_len] = digit;
                sequence_len += 1;

                if sequence_len == target {
                    break 'outer;
                }
            }

            index += 1;
        }
    }

    sum
}

fn main() {
    println!("{}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 304_552_150);
    }
}
