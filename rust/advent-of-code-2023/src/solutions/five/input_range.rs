use std::fmt::Display;

pub struct InputRange {
    pub src_start: usize,
    pub dest_start: usize,
    pub range: usize,
}

impl Display for InputRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source start {}, Destination start {}, with range {}",
            self.src_start, self.dest_start, self.range
        )
    }
}

impl Clone for InputRange {
    fn clone(&self) -> Self {
        InputRange {
            src_start: self.src_start,
            dest_start: self.dest_start,
            range: self.range,
        }
    }
}
