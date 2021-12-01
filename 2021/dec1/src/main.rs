use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut increment_count: u16 = 0;
    let mut previous_depth: u16 = 0;

    for line in reader.lines() {
        let depth: u16 = line?.to_string().parse::<u16>().unwrap();
        if depth > previous_depth {
            increment_count += 1;
        }
        previous_depth = depth;
    }

    println!("{}", increment_count - 1);
    Ok(())
}
