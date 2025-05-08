use std::fmt::Display;

pub enum OverlapType {
    OutsideLeft,
    OutsideRight,
    PartialLeft,
    PartialRight,
    Inner,
    Contain,
    Unknown,
}

pub struct SeedRange {
    pub start: usize,
    pub range: usize,
}

impl Display for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seed start {}, range {}", self.start, self.range)
    }
}

impl SeedRange {
    #[allow(dead_code)]
    fn limit(&self) -> usize {
        self.start + self.range - 1
    }

    #[allow(dead_code)]
    pub fn overlap_type(&self, start: usize, range: usize) -> OverlapType {
        let self_limit = self.limit();
        let range_end = start + range - 1;

        if self.start < start && self_limit < start {
            return OverlapType::OutsideLeft;
        }

        if self.start > range_end && self_limit > range_end {
            return OverlapType::OutsideRight;
        }

        if self.start < start && self_limit >= start && self_limit <= range_end {
            return OverlapType::PartialLeft;
        }

        if self_limit > range_end && self.start >= start && self.start <= range_end {
            return OverlapType::PartialRight;
        }

        if self.start >= start && self_limit <= range_end {
            return OverlapType::Inner;
        }

        if self.start < start && self_limit > range_end {
            return OverlapType::Contain;
        }

        return OverlapType::Unknown;
    }
}
