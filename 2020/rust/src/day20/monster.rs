use super::{
    bitmap::{all_bitmap_variants, Bitmap},
    wave::CandidateGrid,
};
use bitvec::prelude::*;

pub fn subtract_monsters(image: &ResolvedImage) -> Option<ResolvedImage> {
    // Flip and rotate the image until we find a variant with monsters in it
    all_bitmap_variants()
        .filter_map(|(flip, rotation)| {
            find_and_subtract_monsters(image.flip(flip).rotate(rotation))
        })
        .next()
}

const MONSTER_WIDTH: usize = 20;
const MONSTER_WIDTH_U8: u8 = MONSTER_WIDTH as u8;
const MONSTER_HEIGHT: usize = 3;
const MONSTER_HEIGHT_U8: u8 = MONSTER_HEIGHT as u8;

const MONSTER_BITS: [BitArr!(for MONSTER_WIDTH, in usize); MONSTER_HEIGHT] = [
    bitarr![const 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0], //                   #
    bitarr![const 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1], // #    ##    ##    ###
    bitarr![const 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0], //  #  #  #  #  #  #
];

fn find_and_subtract_monsters(mut image: ResolvedImage) -> Option<ResolvedImage> {
    let mut row: BitArr!(for MONSTER_WIDTH, in usize) = bitarr!(0; MONSTER_WIDTH);
    let mut monster_indexes: Vec<usize> = Vec::new();
    let mut found_monster = false;

    // Don't try searching in the places of the image where we can't fit a full monster
    let max_x = image.square - MONSTER_WIDTH_U8;
    let max_y = image.square - MONSTER_HEIGHT_U8;

    // Consider every possible starting position for the monster (top left)
    for y in 0..=max_y {
        for x in 0..=max_x {
            monster_indexes.clear();
            let mut is_monster = true;
            for l in 0..MONSTER_HEIGHT_U8 {
                // Calculate the bit positions in the image which align to this position
                let row_start = image.to_idx(x, y + l);
                let row_end = row_start + MONSTER_WIDTH;
                // Copy this portion of the image
                let dest = &mut row[0..MONSTER_WIDTH];
                dest.clone_from_bitslice(&image.map[row_start..row_end]);
                // Find this template row
                let template = MONSTER_BITS[l as usize].as_bitslice();
                // Find the parts of the image which match the template
                row &= template;
                // If we matched the whole template, we might have a monster
                if row == template {
                    // This row of the template matches, so mark the one bits for removal
                    monster_indexes.extend(row.iter_ones().map(|idx| idx + row_start));
                } else {
                    // No monster here
                    is_monster = false;
                    break;
                }
            }

            // All of the rows matched, so we found a monster, and we can clear it from the image
            if is_monster {
                found_monster = true;
                for &idx in &monster_indexes {
                    image.map.set(idx, false);
                }
            }
        }
    }

    if found_monster {
        Some(image)
    } else {
        None
    }
}

#[derive(Clone)]
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
        let (x, y, square) = (x as usize, y as usize, self.square as usize);
        (y * square) + x
    }

    pub fn roughness(&self) -> i64 {
        self.map.count_ones() as i64
    }
}
