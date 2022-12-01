use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut elf_cals: Vec<usize> = vec![];
    let mut counter: usize = 0;

    for line in reader.lines() {
        match line?.to_string().parse::<usize>() {
            Ok(i) => counter += i,
            Err(_e) => {
                elf_cals.push(counter);
                counter = 0;
            }
        }
    }
    elf_cals.push(counter);

    Ok(*elf_cals.iter().max().unwrap())
}

fn two<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut elf_cals: Vec<usize> = vec![];
    let mut counter: usize = 0;

    for line in reader.lines() {
        match line?.to_string().parse::<usize>() {
            Ok(i) => counter += i,
            Err(_e) => {
                elf_cals.push(counter);
                counter = 0;
            }
        }
    }

    elf_cals.push(counter);
    elf_cals.sort();

    let lasts = &elf_cals[(elf_cals.len()-3)..];
    Ok(lasts.iter().sum())
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
        let cursor = io::Cursor::new(b"1000\n2000\n\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(one(cursor).unwrap(), 24000);
    }

    #[test]
    fn test_two() {
        let cursor = io::Cursor::new(b"1000\n2000\n\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(two(cursor).unwrap(), 45000);
    }
}

