use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: &mut R) -> io::Result<u16> {
    let mut increment_count: u16 = 0;
    let mut previous_depth: u16 = 0;

    for line in reader.lines() {
        let depth: u16 = line?.to_string().parse::<u16>().unwrap();
        if depth > previous_depth {
            increment_count += 1;
        }
        previous_depth = depth;
    }

    Ok(increment_count - 1)
}

fn main() {
    let file_result = File::open("input.txt");
    let file;

    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            println!("File open error! {}", e);
            process::abort();
        }
    }

    let mut reader = BufReader::new(file);

    match one(&mut reader) {
        Err(e) => println!("{:?}", e),
        Ok(n) => println!("{:?}", n),
    }
}
