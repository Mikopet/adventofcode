
pub mod file {
    use lazy_regex::*;
    use std::io::{self, prelude::*};

    pub static OUTPUTS_RE: Lazy<Regex> = lazy_regex!(r"\|.*");
    pub static DIGITS_RE: Lazy<Regex> = lazy_regex!(r"(\b\w{2,4}\b|\b\w{7}\b)");

    pub fn read<R: BufRead>(reader: R) -> io::Result<Vec<usize>> {
        let mut outputs: Vec<usize> = vec![];

        for line in reader.lines() {
            let row: String = line?.to_string();
            let segments = find_digits(&row);
            let output = segments.len();
            outputs.push(output);
        }

        Ok(outputs)
    }

    fn find_digits(s: &str) -> Vec<String> {
        let output = OUTPUTS_RE
            .find_iter(&s)
            .map(|s| s.as_str().to_string())
            .collect::<String>();
        DIGITS_RE.find_iter(&output)
            .map(|d| d.as_str().to_string())
            .collect()
    }
}
