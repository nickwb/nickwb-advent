use crate::day20::bitmap::Bitmap;
use bitvec::{bitarr, BitArr};
use regex::Regex;

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
                    //assert!(t.idx.0 == tiles.len());
                    tiles.push(t);
                }

                let id: i64 = c.get(1).unwrap().as_str().parse().unwrap();

                next_tile = Some(Tile {
                    id,
                    idx: TileIndex(idx),
                    map: TileBitmap::empty(10),
                });
                y = 0;
                idx += 1;
                continue;
            }

            let tile = next_tile
                .as_mut()
                .expect("Expected a tile header to proceed pixel data");
            for (x, c) in l.chars().enumerate() {
                let on = c == '#';
                tile.map.set(x as u8, y, on);
            }

            y += 1;
        }

        if let Some(t) = next_tile {
            //assert!(t.idx.0 == tiles.len());
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
    pixels: BitArr!(for 100, in usize),
}

impl Bitmap for TileBitmap {
    fn empty(_square_length: u8) -> Self {
        //assert_eq!(10, square_length);
        TileBitmap {
            pixels: bitarr![0; 100],
        }
    }

    fn square_length(&self) -> u8 {
        10
    }

    fn is_set(&self, x: u8, y: u8) -> bool {
        self.pixels[Self::to_idx(x, y)]
    }

    fn set(&mut self, x: u8, y: u8, on: bool) {
        self.pixels.set(Self::to_idx(x, y), on);
    }
}

impl TileBitmap {
    fn to_idx(x: u8, y: u8) -> usize {
        ((y as usize) * 10) + (x as usize)
    }
}
