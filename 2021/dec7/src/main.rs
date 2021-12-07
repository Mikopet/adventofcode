fn main() {
    let positions: Vec<u16> = parse(include_str!("../input.txt").to_string());

    println!("{:?}", task(positions));
}

fn parse(s: String) -> Vec<u16> {
    let mut v: Vec<u16> = s.trim().split(",").map(str::parse).map(Result::unwrap).collect();
    v.sort(); v
}

fn task(v: Vec<u16>) -> u32 {
    let m = v[v.len()/2];

    v.iter().fold(0u32, |sum, n| sum + (m.max(*n)-n.min(&m)) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed = parse("16,1,2,0,4,2,7,1,2,14".to_string());
        assert_eq!(parsed, vec![0u16, 1, 1, 2, 2, 2, 4, 7, 14, 16]);
    }

    #[test]
    fn test_task() {
        let v = vec![0u16, 1, 1, 2, 2, 2, 4, 7, 14, 16];
        assert_eq!(task(v), 37);
    }
}
