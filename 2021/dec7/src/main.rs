fn main() {
    let positions: Vec<usize> = parse(include_str!("../input.txt").to_string());

    println!("{:?}", task_one(&mut positions.clone()));
    println!("{:?}", task_two(&positions));
}

fn parse(s: String) -> Vec<usize> {
    s.trim()
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn task_one(v: &mut Vec<usize>) -> usize {
    v.sort();
    let m = v[v.len() / 2];

    v.iter().fold(0, |sum, n| sum + m.max(*n) - n.min(&m))
}

fn task_two(v: &Vec<usize>) -> usize {
    let m = v.iter().sum::<usize>() / v.len();

    std::cmp::min(
        v.iter().map(|n| task_two_consumption(*n, m)).sum(),
        v.iter().map(|n| task_two_consumption(*n, m+1)).sum(),
    )
}

fn task_two_consumption(n: usize, m: usize) -> usize {
    let t = n.max(m) - m.min(n) + 1;

    (1..t).fold(0, |sum, n| sum + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed = parse("16,1,2,0,4,2,7,1,2,14".to_string());
        assert_eq!(parsed, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_task_one() {
        let mut v = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(task_one(&mut v), 37);
    }

    #[test]
    fn test_task_two() {
        let v = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(task_two(&v), 168);
    }

    #[test]
    fn test_task_consumption() {
        assert_eq!(task_two_consumption(16, 5), 66);
        assert_eq!(task_two_consumption(5, 16), 66);
        assert_eq!(task_two_consumption(0, 5), 15);
    }
}
