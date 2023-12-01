use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let cursor = io::Cursor::new(b"1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(one(cursor).unwrap(), 142);
    }
}

