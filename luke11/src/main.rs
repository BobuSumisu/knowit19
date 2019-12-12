use std::fs;

fn solve(input: &str, initial_speed: i64) -> usize {
    let mut speed = initial_speed;
    let mut ice = 0;
    let mut on_mountain = false;

    for (i, ch) in input.chars().enumerate() {
        if ch == 'I' {
            ice += 1;
        } else {
            ice = 0;
        }

        speed = match ch {
            'G' => speed - 27,
            'I' => speed + (12 * ice),
            'A' => speed - 59,
            'S' => speed - 212,
            'F' => {
                if on_mountain {
                    on_mountain = false;
                    speed + 35
                } else {
                    on_mountain = true;
                    speed - 70
                }
            }
            _ => panic!("unexpected char"),
        };

        if speed <= 0 {
            return i + 1;
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    println!("{}", solve(&input, 10_703_437));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "IIGGFFAIISGIFFSGFAAASS";
        assert_eq!(solve(&input, 300), 11);
    }

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&input, 10_703_437), 202_128);
    }
}
