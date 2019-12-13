use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
struct Room {
    x: u64,
    y: u64,
    nord: bool,
    vest: bool,
    syd: bool,
    aust: bool,
}

enum Dir {
    North,
    South,
    East,
    West,
}

fn run_robot(rooms: &[Vec<Room>], order: &[Dir]) -> usize {
    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();

    to_visit.push(&rooms[0][0]);

    while let Some(room) = to_visit.pop() {
        if room == &rooms[499][499] {
            break;
        }

        visited.insert(room);

        let x = room.x as usize;
        let y = room.y as usize;

        for dir in order {
            match dir {
                Dir::North => {
                    if !room.nord && !visited.contains(&rooms[y - 1][x]) {
                        to_visit.push(&rooms[y - 1][x]);
                    }
                }
                Dir::West => {
                    if !room.vest && !visited.contains(&rooms[y][x - 1]) {
                        to_visit.push(&rooms[y][x - 1]);
                    }
                }
                Dir::South => {
                    if !room.syd && !visited.contains(&rooms[y + 1][x]) {
                        to_visit.push(&rooms[y + 1][x]);
                    }
                }
                Dir::East => {
                    if !room.aust && !visited.contains(&rooms[y][x + 1]) {
                        to_visit.push(&rooms[y][x + 1]);
                    }
                }
            }
        }
    }

    visited.len()
}

fn solution() -> usize {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let rooms: Vec<Vec<Room>> = serde_json::from_str(&input).unwrap();

    let arthur = run_robot(&rooms, &[Dir::South, Dir::East, Dir::West, Dir::North]);
    let isaac = run_robot(&rooms, &[Dir::East, Dir::South, Dir::West, Dir::North]);

    if arthur > isaac {
        arthur - isaac
    } else {
        isaac - arthur
    }
}

fn main() {
    println!("{}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 99079);
    }
}
