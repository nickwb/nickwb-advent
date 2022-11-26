use regex::Regex;

use super::{Flip, Rotation};

#[derive(Debug)]
pub struct Inputs {
    pub tiles: Vec<Tile>,
    pub square: u8,
    pub last_idx: u8,
}

impl Inputs {
    pub fn parse(text: &str) -> Self {
        let header = Regex::new(r"^Tile (\d+):$").unwrap();
        let lines = text.lines().map(|l| l.trim()).filter(|l| l.len() > 0);
        let mut tiles: Vec<Tile> = Vec::new();
        let mut next_tile: Option<Tile> = None;
        let mut idx: usize = 0;
        let mut y = 0;

        for l in lines {
            if let Some(c) = header.captures(l) {
                let finished = next_tile.take();
                if let Some(t) = finished {
                    assert!(t.idx.0 == tiles.len());
                    tiles.push(t);
                }

                let id: i64 = c
                    .get(1)
                    .expect("An id")
                    .as_str()
                    .parse()
                    .expect("Id is an int");

                next_tile = Some(Tile {
                    id,
                    idx: TileIndex(idx),
                    map: TileBitmap::empty(),
                });
                y = 0;
                idx += 1;
                continue;
            }

            let tile = next_tile.as_mut().expect("Working on a tile");
            for (x, c) in l.chars().enumerate() {
                let on = c == '#';
                tile.map.set(x as u8, y, on);
            }

            y += 1;
        }

        if let Some(t) = next_tile {
            assert!(t.idx.0 == tiles.len());
            tiles.push(t);
        }

        let square = (tiles.len() as f32).sqrt() as u8;

        Self {
            tiles,
            square,
            last_idx: square - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileIndex(pub usize);

#[derive(Debug)]
pub struct Tile {
    pub id: i64,
    pub idx: TileIndex,
    pub map: TileBitmap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileBitmap {
    pixels: u128,
}

impl TileBitmap {
    fn empty() -> TileBitmap {
        TileBitmap { pixels: 0 }
    }

    pub fn flip(&self, flip: Flip) -> TileBitmap {
        let mut result = TileBitmap::empty();
        for y in 0..10 {
            for x in 0..10 {
                let source = self.is_set(x, y);
                let (dest_x, dest_y) = match flip {
                    Flip::None => (x, y),
                    Flip::Horizontal => (9 - x, y),
                    Flip::Vertical => (x, 9 - y),
                    Flip::Both => (9 - x, 9 - y),
                };
                result.set(dest_x, dest_y, source);
            }
        }
        result
    }

    pub fn rotate(&self, rotation: Rotation) -> TileBitmap {
        let mut result = TileBitmap::empty();
        for y in 0..10 {
            for x in 0..10 {
                let source = self.is_set(x, y);
                let (dest_x, dest_y) = match rotation {
                    Rotation::R0 => (x, y),
                    Rotation::R180 => (9 - x, 9 - y),
                    Rotation::R90 => (9 - y, x),
                    Rotation::R270 => (y, 9 - x),
                };
                result.set(dest_x, dest_y, source);
            }
        }
        result
    }

    // fn to_string(&self) -> String {
    //     let mut buffer = String::with_capacity(10 * (10 + 2));
    //     for y in 0..10 {
    //         for x in 0..10 {
    //             let sym = if self.is_set(x, y) { '#' } else { '.' };
    //             buffer.push(sym);
    //         }
    //         buffer.push_str("\r\n");
    //     }

    //     buffer
    // }

    pub fn is_set(&self, x: u8, y: u8) -> bool {
        (TileBitmap::to_mask(x, y) & self.pixels) > 0
    }

    fn set(&mut self, x: u8, y: u8, on: bool) {
        let mask = TileBitmap::to_mask(x, y);
        if on {
            self.pixels |= mask;
        } else {
            self.pixels &= !mask;
        }
    }

    fn to_mask(x: u8, y: u8) -> u128 {
        1 << TileBitmap::to_idx(x, y)
    }

    fn to_idx(x: u8, y: u8) -> u8 {
        assert!(x <= 9 && y <= 9);
        ((9 - y) * 10) + (9 - x)
    }
}
