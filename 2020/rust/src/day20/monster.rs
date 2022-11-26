use super::{bitmap::Bitmap, wave::CandidateGrid};
use bitvec::{bitvec, vec::BitVec};

pub struct ResolvedImage {
    map: BitVec,
    square: u8,
}

impl<'a> From<CandidateGrid<'a>> for ResolvedImage {
    fn from(grid: CandidateGrid<'a>) -> Self {
        let square = grid.expanded_input.inputs.square;
        let pixel_square = 8 * square;

        let mut result = Self::empty(pixel_square);

        for y in 0..pixel_square {
            for x in 0..pixel_square {
                let (grid_x, grid_y) = (x / 8, y / 8);
                let (pix_x, pix_y) = ((x % 8) + 1, (y % 8) + 1);
                let variant = grid.variant_at(grid_x, grid_y);
                let pixel = variant.map.is_set(pix_x, pix_y);
                result.set(x, y, pixel);
            }
        }

        result
    }
}

impl Bitmap for ResolvedImage {
    fn empty(square_length: u8) -> Self {
        let square = square_length as usize;
        ResolvedImage {
            map: bitvec![0; square * square],
            square: square_length,
        }
    }

    fn square_length(&self) -> u8 {
        self.square
    }

    fn is_set(&self, x: u8, y: u8) -> bool {
        self.map[self.to_idx(x, y)]
    }

    fn set(&mut self, x: u8, y: u8, on: bool) {
        let idx = self.to_idx(x, y);
        self.map.set(idx, on);
    }
}

impl ResolvedImage {
    fn to_idx(&self, x: u8, y: u8) -> usize {
        assert!(x < self.square && y < self.square);
        let (x, y, square) = (x as usize, y as usize, self.square as usize);
        let last = square - 1;
        ((last - y) * square) + (last - x)
    }
}
