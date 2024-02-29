use std::ops::RangeInclusive;

pub fn inv_range<Idx: std::cmp::PartialOrd>(start: Idx, end: Idx) -> RangeInclusive<Idx> {
    if start > end {
        return end..=start;
    }
    start..=end
}