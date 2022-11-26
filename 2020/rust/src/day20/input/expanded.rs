use crate::day20::bitmap::Bitmap;

use super::{Flip, Inputs, Rotation, Tile, TileBitmap, TileIndex};
use itertools::Itertools;
use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
pub struct Variant {
    pub id: VariantId,
    pub tile: TileIndex,
    pub map: TileBitmap,
    edges: Edges,
}

type AdjacentMap = HashMap<(VariantId, Side), Vec<VariantId>>;

#[derive(Debug)]
pub struct ExpandedInput<'a> {
    pub inputs: &'a Inputs,
    pub variants: Vec<Variant>,
    pub adjacent: AdjacentMap,
}

impl<'a> ExpandedInput<'a> {
    pub fn build(inputs: &'a Inputs) -> Self {
        let variants: Vec<Variant> = inputs
            .tiles
            .iter()
            .flat_map(|t| ExpandedInput::make_variants(t))
            .enumerate()
            .map(|(idx, mut v)| {
                v.id = VariantId(idx);
                v
            })
            .collect();
        let adjacent = ExpandedInput::calculate_adjacents(&variants);
        Self {
            inputs,
            variants,
            adjacent,
        }
    }

    pub fn can_connect_on(&self, variant: &Variant, side: Side) -> bool {
        match self.adjacent.get(&(variant.id, side)) {
            Some(adj) => !adj.is_empty(),
            None => false,
        }
    }

    const ALL_FLIPS: [Flip; 4] = [Flip::None, Flip::Horizontal, Flip::Vertical, Flip::Both];
    const ALL_ROTATIONS: [Rotation; 4] =
        [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];

    fn make_variants(tile: &'a Tile) -> impl Iterator<Item = Variant> + 'a {
        ExpandedInput::ALL_FLIPS
            .iter()
            .cloned()
            .cartesian_product(ExpandedInput::ALL_ROTATIONS.iter().cloned())
            .map(move |(f, r)| {
                let map = tile.map.flip(f).rotate(r);
                Variant {
                    id: VariantId(0), // We'll fix this later
                    tile: tile.idx,
                    map,
                    edges: Edges::from_bitmap(&map),
                }
            })
            .unique_by(|v| v.map)
    }

    fn calculate_adjacents(variants: &[Variant]) -> AdjacentMap {
        let grouped = variants
            .iter()
            .flat_map(|a| variants.iter().map(move |b| (a, b)))
            .filter(|(a, b)| a.tile != b.tile)
            .flat_map(|(a, b)| {
                ExpandedInput::get_adjoining_edges(a, b).map(move |e| (a.id, e.side, b.id))
            })
            .group_by(|(a_id, side, _)| (*a_id, *side));

        let map: AdjacentMap = grouped
            .into_iter()
            .map(|g| (g.0, g.1.map(|x| x.2).collect()))
            .collect();

        map
    }

    fn get_adjoining_edges(a: &'a Variant, b: &'a Variant) -> impl Iterator<Item = &'a Edge> {
        a.edges
            .into_iter()
            .filter(move |e| e.pixels == b.edges[e.side.inverse()].pixels)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    fn inverse(&self) -> Side {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug)]
pub struct Edges {
    top: Edge,
    bottom: Edge,
    left: Edge,
    right: Edge,
}

impl Edges {
    fn from_bitmap(bitmap: &TileBitmap) -> Edges {
        let mut top = Edge::new(Side::Top);
        let mut bottom = Edge::new(Side::Bottom);
        let mut left = Edge::new(Side::Left);
        let mut right = Edge::new(Side::Right);
        for i in 0..10 {
            top.set(i, bitmap.is_set(i, 0));
            bottom.set(i, bitmap.is_set(i, 9));
            left.set(i, bitmap.is_set(0, i));
            right.set(i, bitmap.is_set(9, i));
        }
        Edges {
            top,
            bottom,
            left,
            right,
        }
    }
}

impl Index<Side> for Edges {
    type Output = Edge;

    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Top => &self.top,
            Side::Bottom => &self.bottom,
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }
}

pub struct EdgesIterator<'a> {
    edges: &'a Edges,
    idx: usize,
}

impl<'a> Iterator for EdgesIterator<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.idx {
            0 => Some(&self.edges.top),
            1 => Some(&self.edges.bottom),
            2 => Some(&self.edges.left),
            3 => Some(&self.edges.right),
            _ => None,
        };
        if item.is_some() {
            self.idx += 1;
        }
        item
    }
}

impl<'a> IntoIterator for &'a Edges {
    type Item = &'a Edge;
    type IntoIter = EdgesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EdgesIterator {
            idx: 0,
            edges: self,
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    pixels: u16,
    side: Side,
}

impl Edge {
    fn new(side: Side) -> Self {
        Edge { pixels: 0, side }
    }

    fn set(&mut self, w: u8, on: bool) {
        let mask = Edge::to_mask(w);
        if on {
            self.pixels |= mask;
        } else {
            self.pixels &= !mask;
        }
    }

    fn to_mask(w: u8) -> u16 {
        1 << (w as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariantId(pub usize);
