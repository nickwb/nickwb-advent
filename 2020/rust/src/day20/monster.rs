use super::wave::CandidateGrid;

pub struct ResolvedImage {
    map: ResolvedBitmap,
}

impl<'a> From<CandidateGrid<'a>> for ResolvedImage {
    fn from(grid: CandidateGrid<'a>) -> Self {
        todo!()
    }
}

struct ResolvedBitmap {}
