pub mod vents {
    #[derive(Debug)]
    pub struct Segment {
        pub from: (u16, u16),
        pub to: (u16, u16),
    }

    pub fn count_highs(segments: &Vec<Segment>, diagonal: bool) -> u32 {
        let mut points: Vec<(u16, u16)> = vec![];

        for segment in segments {
            if diagonal {
                points.append(&mut break_down_segment_diagonal(segment))
            } else {
                points.append(&mut break_down_segment_linear(segment))
            }
        }

        count_duplicates(points) as u32
    }

    fn count_duplicates(points: Vec<(u16, u16)>) -> usize {
        use std::collections::HashMap;
        let mut value_counts: HashMap<(u16, u16), u8> = HashMap::new();

        for item in points.iter() {
            *value_counts.entry(*item).or_insert(0) += 1;
        }

        value_counts
            .into_iter()
            .filter(|(_k, v)| *v > 1)
            .map(|(k, _v)| k)
            .count()
    }

    fn break_down_segment_linear(segment: &Segment) -> Vec<(u16, u16)> {
        let mut points: Vec<(u16, u16)> = vec![];

        let (from_x, from_y, to_x, to_y) =
            (segment.from.0, segment.from.1, segment.to.0, segment.to.1);

        if from_y == to_y {
            for i in from_x.min(to_x)..from_x.max(to_x) + 1 {
                points.push((i, from_y));
            }
        }

        if from_x == to_x {
            for i in from_y.min(to_y)..from_y.max(to_y) + 1 {
                points.push((from_x, i));
            }
        }

        points
    }

    fn break_down_segment_diagonal(segment: &Segment) -> Vec<(u16, u16)> {
        let points: Vec<(u16, u16)> = vec![];

        points
    }
}

pub mod file {
    use super::vents::Segment;
    use lazy_regex::*;
    use std::io::{self, prelude::*};

    pub static DIGITS_RE: Lazy<Regex> = lazy_regex!(r"\d+");

    pub fn read<R: BufRead>(reader: R) -> io::Result<Vec<Segment>> {
        let mut segments: Vec<Segment> = vec![];

        for line in reader.lines() {
            let row: String = line?.to_string();
            let coords = find_digits(&row);

            let segment = Segment {
                from: (coords[0], coords[1]).into(),
                to: (coords[2], coords[3]).into(),
            };

            segments.push(segment);
        }

        Ok(segments)
    }

    fn find_digits(s: &str) -> Vec<u16> {
        DIGITS_RE
            .find_iter(&s)
            .filter_map(|d| d.as_str().parse::<u16>().ok())
            .collect()
    }
}
