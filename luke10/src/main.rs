use chrono::{Date, Datelike, TimeZone, Utc, Weekday};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res},
    multi::many1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Unit {
    Meter,
    Millilitre,
}

#[derive(Debug, Clone, Copy)]
enum Product {
    Toothpaste,
    Shampoo,
    ToiletPaper,
}

#[derive(Debug)]
struct LogEntry {
    date: Date<Utc>,
    toothpaste: u64,
    shampoo: u64,
    toilet_paper: u64,
}

fn parse_month(i: &str) -> IResult<&str, u32> {
    alt((
        map(tag("Jan"), |_| 1),
        map(tag("Feb"), |_| 2),
        map(tag("Mar"), |_| 3),
        map(tag("Apr"), |_| 4),
        map(tag("May"), |_| 5),
        map(tag("Jun"), |_| 6),
        map(tag("Jul"), |_| 7),
        map(tag("Aug"), |_| 8),
        map(tag("Sep"), |_| 9),
        map(tag("Oct"), |_| 10),
        map(tag("Nov"), |_| 11),
        map(tag("Dec"), |_| 12),
    ))(i)
}

fn parse_u64(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}

fn parse_date_line(i: &str) -> IResult<&str, Date<Utc>> {
    match tuple((parse_month, char(' '), parse_u64, char(':'), char('\n')))(i) {
        Ok((remaining, (month, _, day, _, _))) => Ok((remaining, Utc.ymd(2018, month, day as u32))),
        Err(e) => Err(e),
    }
}

fn parse_unit(i: &str) -> IResult<&str, Unit> {
    alt((
        map(tag("ml"), |_| Unit::Millilitre),
        map(tag("meter"), |_| Unit::Meter),
    ))(i)
}

fn parse_product(i: &str) -> IResult<&str, Product> {
    alt((
        map(tag("tannkrem"), |_| Product::Toothpaste),
        map(tag("sjampo"), |_| Product::Shampoo),
        map(tag("toalettpapir"), |_| Product::ToiletPaper),
    ))(i)
}

fn parse_product_line(i: &str) -> IResult<&str, (Product, u64)> {
    match tuple((
        tag("\t* "),
        parse_u64,
        char(' '),
        parse_unit,
        char(' '),
        parse_product,
        char('\n'),
    ))(i)
    {
        Ok((remaining, (_, amount, _, _, _, product, _))) => Ok((remaining, (product, amount))),
        Err(e) => Err(e),
    }
}

fn parse_log_entry(i: &str) -> IResult<&str, LogEntry> {
    let (remaining, (date, prods)) = tuple((parse_date_line, many1(parse_product_line)))(i)?;

    let mut entry = LogEntry {
        date,
        toothpaste: 0,
        shampoo: 0,
        toilet_paper: 0,
    };

    for prod in prods {
        match prod.0 {
            Product::Toothpaste => {
                entry.toothpaste = prod.1;
            }
            Product::Shampoo => {
                entry.shampoo = prod.1;
            }
            Product::ToiletPaper => {
                entry.toilet_paper = prod.1;
            }
        }
    }

    Ok((remaining, entry))
}

fn parse_input(input: &str) -> Vec<LogEntry> {
    all_consuming(many1(parse_log_entry))(input).unwrap().1
}

fn solution(entries: &[LogEntry]) -> u64 {
    let (toothpaste, shampoo, toilet_paper) = entries.iter().fold((0, 0, 0), |(t, s, tp), e| {
        (t + e.toothpaste, s + e.shampoo, tp + e.toilet_paper)
    });

    let (toothpaste, shampoo, toilet_paper) = (toothpaste / 125, shampoo / 300, toilet_paper / 25);

    let sunday_shampoo = entries
        .iter()
        .filter_map(|e| {
            if e.date.weekday() == Weekday::Sun {
                Some(e.shampoo)
            } else {
                None
            }
        })
        .sum::<u64>();

    let wednesday_toilet_paper = entries
        .iter()
        .filter_map(|e| {
            if e.date.weekday() == Weekday::Wed {
                Some(e.toilet_paper)
            } else {
                None
            }
        })
        .sum::<u64>();

    toothpaste * shampoo * toilet_paper * sunday_shampoo * wednesday_toilet_paper
}

fn main() {
    let input = include_str!("../input/input.txt");
    let entries = parse_input(input);
    println!("{}", solution(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let entries = parse_input(include_str!("../input/input.txt"));
        assert_eq!(1_888_326_000, solution(&entries));
    }
}
