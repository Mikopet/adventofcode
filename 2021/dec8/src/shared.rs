pub mod file {
    use lazy_regex::*;
    use std::io::{self, prelude::*};

    pub static OUTPUTS_RE: Lazy<Regex> = lazy_regex!(r"\b\w+\b");

    pub fn read<R: BufRead>(reader: R) -> io::Result<Vec<Vec<String>>> {
        let mut outputs: Vec<Vec<String>> = vec![];

        for line in reader.lines() {
            let row: String = line?.to_string();
            let segments = find_digits(&row);
            outputs.push(segments);
        }

        Ok(outputs)
    }

    fn find_digits(s: &str) -> Vec<String> {
        OUTPUTS_RE
            .find_iter(&s)
            .map(|s| s.as_str().to_string())
            .collect()
    }
}
