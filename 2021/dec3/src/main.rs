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
            if counter.len() < bit + 1 {
                counter.push(0);
            }
            if diagnostic.chars().nth(bit).unwrap().to_string() == "1" {
                counter[bit] += 1;
            }
        }

        data_length += 1;
    }

    for sum in counter {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if sum * 2 > data_length {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    Ok(gamma * epsilon)
}

fn two_iterator(length: usize, mut togo: Vec<String>, direction: &str) -> u32 {
    for bit in 0..length {
        let mut ones: Vec<String> = vec![];
        let mut zeros: Vec<String> = vec![];

        for diagnostic in &togo {
            if diagnostic.chars().nth(bit).unwrap().to_string() == "1" {
                ones.push(diagnostic.to_string())
            } else {
                zeros.push(diagnostic.to_string())
            }
        }

        match direction {
            "o2" => {
                if zeros.len() > ones.len() {
                    togo = zeros.clone();
                } else {
                    togo = ones.clone();
                }
            }
            "co2" => {
                if ones.len() < zeros.len() {
                    togo = ones.clone();
                } else {
                    togo = zeros.clone();
                }
            }
            _ => (),
        }

        if togo.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&togo[0], 2).unwrap()
}

fn two<R: BufRead>(reader: R) -> io::Result<u32> {
    let mut togo: Vec<String> = vec![];
    let mut len = 0;

    for line in reader.lines() {
        let diagnostic: String = line?.to_string();
        len = diagnostic.len();

        togo.push(diagnostic);
    }

    let oxygen: u32 = two_iterator(len, togo.clone(), "o2");
    let co2: u32 = two_iterator(len, togo.clone(), "co2");

    Ok(oxygen * co2)
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
        let cursor = io::Cursor::new(
            b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(one(cursor).unwrap(), 22 * 9);
    }

    #[test]
    fn test_two() {
        let cursor = io::Cursor::new(
            b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(two(cursor).unwrap(), 23 * 10);
    }
}
