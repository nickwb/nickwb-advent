use super::input::{Flip, Rotation};

const ALL_FLIPS: [Flip; 4] = [Flip::None, Flip::Horizontal, Flip::Vertical, Flip::Both];
const ALL_ROTATIONS: [Rotation; 4] = [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];

pub fn all_bitmap_variants() -> impl Iterator<Item = (Flip, Rotation)> {
    ALL_FLIPS
        .iter()
        .flat_map(|&f| ALL_ROTATIONS.iter().map(move |&r| (f, r)))
}

pub trait Bitmap {
    fn empty(square_length: u8) -> Self;
    fn square_length(&self) -> u8;
    fn is_set(&self, x: u8, y: u8) -> bool;
    fn set(&mut self, x: u8, y: u8, on: bool);

    fn flip(&self, flip: Flip) -> Self
    where
        Self: Sized,
    {
        let length = self.square_length();
        let last = length - 1;
        let mut result = Self::empty(length);

        for y in 0..=last {
            for x in 0..=last {
                let source = self.is_set(x, y);
                let (dest_x, dest_y) = match flip {
                    Flip::None => (x, y),
                    Flip::Horizontal => (last - x, y),
                    Flip::Vertical => (x, last - y),
                    Flip::Both => (last - x, last - y),
                };
                result.set(dest_x, dest_y, source);
            }
        }
        result
    }

    fn rotate(&self, rotation: Rotation) -> Self
    where
        Self: Sized,
    {
        let length = self.square_length();
        let last = length - 1;
        let mut result = Self::empty(length);

        for y in 0..=last {
            for x in 0..=last {
                let source = self.is_set(x, y);
                let (dest_x, dest_y) = match rotation {
                    Rotation::R0 => (x, y),
                    Rotation::R180 => (last - x, last - y),
                    Rotation::R90 => (last - y, x),
                    Rotation::R270 => (y, last - x),
                };
                result.set(dest_x, dest_y, source);
            }
        }
        result
    }
}
