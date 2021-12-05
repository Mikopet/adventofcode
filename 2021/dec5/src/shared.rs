pub mod vents {
    #[derive(Debug)]
    pub struct Segment {
        pub from: (u16, u16),
        pub to: (u16, u16),
    }

    pub fn count_intersections(segments: Vec<Segment>) -> u32 {
        let mut line: Option<Segment> = None;
        let mut intersection_count: u32 = 0;

        for segment in segments {
            println!("CURRENT {:?}", segment);
            if line.is_some() {
                let intersection: Option<(u16, u16)> = None;

                if intersection.is_some() {
                    println!("INTERSECT {:?}", intersection);
                    intersection_count += 1;
                }
            }

            line = Some(segment);
        }

        intersection_count
    }
}

pub mod file {
    use lazy_regex::*;
    use std::io::{self, prelude::*};
    use super::vents::Segment;

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
