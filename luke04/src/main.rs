use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let coords = parse_input(&input);
    println!("{}", Snail::new().walk_path(&coords));
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .skip(1)
        .filter_map(|l| l.parse().ok())
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    const fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<u64> = s.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Self { x: v[0], y: v[1] })
    }
}

struct Snail {
    pos: Point,
    slime: HashMap<Point, u64>,
}

impl Snail {
    fn new() -> Self {
        Self {
            pos: Point::new(0, 0),
            slime: HashMap::new(),
        }
    }

    fn visit(&mut self, pos: Point) -> u64 {
        if let Some(slime) = self.slime.get_mut(&pos) {
            *slime += 1;
            *slime
        } else {
            self.slime.insert(pos, 1);
            1
        }
    }

    fn move_to(&mut self, dest: &Point) -> u64 {
        let mut time_used = 0;

        while self.pos.x != dest.x {
            if self.pos.x < dest.x {
                self.pos.x += 1;
            } else {
                self.pos.x -= 1;
            }
            time_used += self.visit(self.pos.clone());
        }

        while self.pos.y != dest.y {
            if self.pos.y < dest.y {
                self.pos.y += 1;
            } else {
                self.pos.y -= 1;
            }
            time_used += self.visit(self.pos.clone());
        }

        time_used
    }

    fn walk_path(&mut self, path: &[Point]) -> u64 {
        path.iter().map(|p| self.move_to(p)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let coords = vec![Point::new(1, 3), Point::new(1, 0), Point::new(3, 2)];
        assert_eq!(14, Snail::new().walk_path(&coords));
    }

    #[test]
    fn test_solution() {
        let data = parse_input(&fs::read_to_string("input/input.txt").unwrap());
        assert_eq!(394426, Snail::new().walk_path(&data));
    }
}
