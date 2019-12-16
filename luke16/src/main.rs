fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut birte = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'B' => {
                        birte = (x, y);
                        ' '
                    }
                    ch => ch,
                })
                .collect()
        })
        .collect();
    (map, birte)
}

fn solution(map: &[Vec<char>], birte: (usize, usize)) -> usize {
    let mut num_turns = 0;
    let mut y = birte.1 as i64;
    let mut dy = -1;

    for x in birte.0..map[0].len() {
        if map[(y + (3 * dy)) as usize][x] == '#' {
            num_turns += 1;
            dy = -dy;
        } else {
            y += dy;
        }
    }

    num_turns + 1
}

fn main() {
    let (map, birte) = parse_input(include_str!("../input/input.txt"));
    println!("{}", solution(&map, birte));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution() {
        let (map, birte) = parse_input(include_str!("../input/input.txt"));
        assert_eq!(solution(&map, birte), 117);
    }
}
