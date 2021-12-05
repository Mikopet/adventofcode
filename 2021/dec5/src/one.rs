use crate::shared::*;
use vents::Segment;

pub fn one(segments: Vec<Segment>) -> u32 {
    vents::count_highs(segments)
}
