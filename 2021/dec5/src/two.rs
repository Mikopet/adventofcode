use crate::shared::*;
use vents::Segment;

pub fn two(segments: &Vec<Segment>) -> u32 {
    vents::count_highs(segments, false)
}
