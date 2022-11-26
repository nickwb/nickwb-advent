use super::input::{Flip, Rotation};

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

    fn to_string(&self) -> String {
        let length = self.square_length() as usize;
        let mut buffer = String::with_capacity((length * 3) * length);

        for y in 0..length {
            for x in 0..length {
                let bit = self.is_set(x as u8, y as u8);
                buffer.push(if bit { '#' } else { '.' });
            }

            buffer.push_str("\r\n");
        }

        buffer
    }
}
