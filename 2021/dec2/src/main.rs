use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: R) -> io::Result<u32> {
    let mut distance: u32 = 0;
    let mut depth: u32 = 0;

    for line in reader.lines() {
        let command: String = line?.to_string();

        let mut words = command.split_whitespace();
        let direction: &str = words.next().unwrap();
        let value: u32 = words.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "forward" => distance += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => ()
        }
    }

    Ok(distance * depth)
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
        let cursor = io::Cursor::new(b"forward 5\ndown 3\nup 1");
        assert_eq!(one(cursor).unwrap(), 5*(3-1));
    }
}
