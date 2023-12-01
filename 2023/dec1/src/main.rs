use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;
use onig::*;

fn one<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut nums: Vec<usize> = vec![];

    for line in reader.lines() {
        let mut digits: String = String::new();
        let mut row = line?.clone();

        while let Some(c) = &row.remove(0).into() {
            match c.to_string().parse::<usize>() {
                Ok(i) => {digits += &i.to_string(); row = i.to_string()+&row; break},
                Err(_e) => {}
            }
        }

        while let Some(c) = &row.pop() {
            match c.to_string().parse::<usize>() {
                Ok(i) => {digits += &i.to_string(); break},
                Err(_e) => {}
            }
        }

        nums.push(digits.parse::<usize>().ok().unwrap());
    }

    Ok(nums.iter().sum())
}

fn two<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut nums: Vec<usize> = vec![];
    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    for line in reader.lines() {
        let row = line?.clone();
        let mut digits: String = String::new();
        let mut results = vec![];

        for digit in re.captures_iter(&row) {
            results.push(digit.at(1).unwrap());
        }

        let first = results[0];
        let last = results[results.len()-1];

        match first.parse::<usize>() {
            Ok(i) => digits += &i.to_string(),
            Err(_e) => {
                digits += &convert(first).to_string();
            }
        }

        match last.parse::<usize>() {
            Ok(i) => digits += &i.to_string(),
            Err(_e) => {
                digits += &convert(last).to_string();
            }
        }

        nums.push(digits.parse::<usize>().ok().unwrap());
    }

    Ok(nums.iter().sum())
}

fn convert(s: &str) -> usize {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _  => 0
    }
}

fn main() {
    let file;

    match File::open("input.txt") {
        Ok(f) => file = f,
        Err(e) => {
            println!("File open error! {}", e);
            process::abort();
        }
    }

    let mut reader = BufReader::new(&file);

    match one(&mut reader) {
        Err(e) => println!("{:?}", e),
        Ok(n) => println!("{:?}", n),
    }

    (&mut &file).rewind().ok();

    match two(&mut reader) {
        Err(e) => println!("{:?}", e),
        Ok(n) => println!("{:?}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let cursor = io::Cursor::new(b"1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(one(cursor).unwrap(), 142);
    }

    #[test]
    fn test_two() {
        let cursor = io::Cursor::new(b"two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        assert_eq!(two(cursor).unwrap(), 281);
    }

    #[test]
    fn test_two_edge_case() {
        let cursor = io::Cursor::new(b"one1eight\noneight");
        assert_eq!(two(cursor).unwrap(), 18+18);
    }
}

