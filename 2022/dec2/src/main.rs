use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut points: usize = 0;

    for line in reader.lines() {
        let s: String = line?.to_string();
        let o = s.chars().next().unwrap(); // opponent

        match s.chars().last().unwrap() {
            'X' => { // Rock
                points += 1;
                if o == 'A' { points += 3 } // Rock
                if o == 'C' { points += 6 } // Scissors
            },
            'Y' => { // Paper
                points += 2;
                if o == 'A' { points += 6 } // Rock
                if o == 'B' { points += 3 } // Paper
            },
            'Z' => { // Scissors
                points += 3;
                if o == 'B' { points += 6 } // Paper
                if o == 'C' { points += 3 } // Scissors
            },
            i   => println!("{}", i)
        }
    }

    Ok(points)
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
        let cursor = io::Cursor::new(b"A Y\nB X\nC Z");
        assert_eq!(one(cursor).unwrap(), 15);
    }
}

