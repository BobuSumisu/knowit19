use std::str::FromStr;

fn msd(n: i64) -> i64 {
    let mut n = n.abs();
    let mut r = 0;

    while n != 0 {
        r = n % 10;
        n /= 10;
    }

    r
}

const fn lsd(n: i64) -> i64 {
    n.abs() % 10
}

fn opp7(n: i64) -> i64 {
    for i in 0..14 {
        let x = n + i;
        if (x < 0 && x % -10 == -7) || x % 10 == 7 {
            return x;
        }
    }
    unreachable!();
}

fn gangemsd(n: i64) -> i64 {
    n * msd(n)
}

fn delemsd(n: i64) -> i64 {
    n.div_euclid(msd(n))
}

fn pluss1tilpar(n: i64) -> i64 {
    let mut out = 0;
    let mut i = 0;
    let mut x = n.abs();

    while x != 0 {
        let mut r = x % 10;
        if x % 2 == 0 {
            r += 1;
        }
        out += r * (10_i64.pow(i));
        i += 1;
        x /= 10;
    }

    if n < 0 {
        -out
    } else {
        out
    }
}

fn trekk1fraodde(n: i64) -> i64 {
    let mut out = 0;
    let mut i = 0;
    let mut x = n.abs();

    while x != 0 {
        let mut r = x % 10;
        if x % 2 == 1 {
            r -= 1;
        }
        out += r * (10_i64.pow(i));
        i += 1;
        x /= 10;
    }

    if n < 0 {
        -out
    } else {
        out
    }
}

fn reverse(n: i64) -> i64 {
    let mut out = 0;
    let mut x = n.abs();

    while x != 0 {
        out = (out * 10) + (x % 10);
        x /= 10;
    }

    if n < 0 {
        -out
    } else {
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Pluss(i64),
    Minus(i64),
    ReverserSiffer,
    Opp7,
    GangeMsd,
    DeleMsd,
    Pluss1TilPar,
    Trekk1FraOdde,
    RoterPar,
    RoterOdde,
    RoterAlle,
    Stopp,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "PLUSS4" => Self::Pluss(4),
            "PLUSS101" => Self::Pluss(101),
            "MINUS9" => Self::Minus(9),
            "MINUS1" => Self::Minus(1),
            "REVERSERSIFFER" => Self::ReverserSiffer,
            "OPP7" => Self::Opp7,
            "GANGEMSD" => Self::GangeMsd,
            "DELEMSD" => Self::DeleMsd,
            "PLUSS1TILPAR" => Self::Pluss1TilPar,
            "TREKK1FRAODDE" => Self::Trekk1FraOdde,
            "ROTERPAR" => Self::RoterPar,
            "ROTERODDE" => Self::RoterOdde,
            "ROTERALLE" => Self::RoterAlle,
            "STOPP" => Self::Stopp,
            _ => return Err(format!("unknown: {}", s)),
        };
        Ok(r)
    }
}

#[derive(Clone, Debug)]
struct Wheel {
    slots: Vec<Op>,
    index: usize,
}

impl Wheel {
    fn op(&self) -> Op {
        self.slots[self.index]
    }

    fn rotate(&mut self) {
        self.index += 1;
        self.index %= self.slots.len();
    }
}

fn parse_input(input: &str) -> Vec<Wheel> {
    input
        .lines()
        .map(|l| {
            let x = l.split(':').last().unwrap();
            let slots = x.split(',').map(|n| n.trim().parse().unwrap()).collect();
            Wheel { slots, index: 0 }
        })
        .collect()
}

fn main() {
    let wheels = parse_input(include_str!("../input/input.txt"));
    println!("{}", find_largest(&wheels));
}

fn find_largest(wheels: &[Wheel]) -> i64 {
    let mut largest = 0;

    for n in 0..=10 {
        let x = play(wheels, n);
        if x > largest {
            largest = x;
        }
    }

    largest
}

fn play(init: &[Wheel], n: i64) -> i64 {
    let mut n = n;
    let mut wheels = init.to_vec();

    loop {
        let i = lsd(n) as usize;

        match wheels[i].op() {
            Op::Pluss(x) => {
                n += x;
            }
            Op::Minus(x) => {
                n -= x;
            }
            Op::ReverserSiffer => {
                n = reverse(n);
            }
            Op::Opp7 => {
                n = opp7(n);
            }
            Op::GangeMsd => {
                n = gangemsd(n);
            }
            Op::DeleMsd => {
                n = delemsd(n);
            }
            Op::Pluss1TilPar => {
                n = pluss1tilpar(n);
            }
            Op::Trekk1FraOdde => {
                n = trekk1fraodde(n);
            }
            Op::RoterPar => {
                for j in (0..10).step_by(2) {
                    wheels[j].rotate();
                }
            }
            Op::RoterOdde => {
                for j in (1..10).step_by(2) {
                    wheels[j].rotate();
                }
            }
            Op::RoterAlle => {
                for j in 0..10 {
                    wheels[j].rotate();
                }
            }
            Op::Stopp => break,
        };

        wheels[i].rotate();
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msd() {
        assert_eq!(1, msd(1234));
        assert_eq!(1, msd(-1234));
    }

    #[test]
    fn test_lsd() {
        assert_eq!(4, lsd(1234));
        assert_eq!(4, lsd(-1234));
    }

    #[test]
    fn test_opp7() {
        assert_eq!(27, opp7(21));
        assert_eq!(-7, opp7(-13));
        assert_eq!(17, opp7(17));
    }

    #[test]
    fn test_gangemsd() {
        assert_eq!(46, gangemsd(23));
        assert_eq!(-93, gangemsd(-31));
        assert_eq!(25, gangemsd(5));
    }

    #[test]
    fn test_delemsd() {
        assert_eq!(11, delemsd(23));
        assert_eq!(-11, delemsd(-33));
    }

    #[test]
    fn test_pluss1tilpar() {
        assert_eq!(131, pluss1tilpar(120));
        assert_eq!(-1335, pluss1tilpar(-1234));
        assert_eq!(9999, pluss1tilpar(8888));
    }

    #[test]
    fn test_trekk1fraodde() {
        assert_eq!(224, trekk1fraodde(1234));
        assert_eq!(-224, trekk1fraodde(-1234));
        assert_eq!(22, trekk1fraodde(23));
        assert_eq!(-222_222, trekk1fraodde(-232_323));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(4321, reverse(1234));
        assert_eq!(-4321, reverse(-1234));
    }

    #[test]
    fn test_solution() {
        let wheels = parse_input(include_str!("../input/input.txt"));
        assert_eq!(84205, find_largest(&wheels));
    }
}
