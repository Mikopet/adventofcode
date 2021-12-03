use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: R) -> io::Result<u32> {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let mut counter: Vec<u32> = vec![];
    let mut data_length: u32 = 0;

    for line in reader.lines() {
        let diagnostic: String = line?.to_string();
        let len = diagnostic.len();

        for bit in 0..len {
            if counter.len() < bit+1 { counter.push(0); }
            if diagnostic.chars().nth(bit).unwrap().to_string() == "1" { counter[bit] += 1; }
        }

        data_length += 1;
    }

    for sum in counter {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if sum*2 > data_length { gamma += 1; } else { epsilon += 1; }
    }

    Ok(gamma * epsilon)
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
        let cursor = io::Cursor::new(b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        assert_eq!(one(cursor).unwrap(), 22*9);
    }
}
