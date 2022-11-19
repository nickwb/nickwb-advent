use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
};

use bitvec::{prelude::BitArray, BitArr};
use itertools::Itertools;
use regex::Regex;

pub fn run_day_twenty() {
    let inputs = inputs();
    println!("Day 20, Part 1: {}", calculate_part_1(&inputs));
    // println!("Day 20, Part 2: {}", calculate_part_2(&mut inputs));
}

fn calculate_part_1(input: &Day20) -> i64 {
    let model = Model::build(input);
    let grid = solve_grid(&model).expect("A solution");
    let last = input.square - 1;
    let a = grid.get(0, 0).get_tile_id(&model).expect("An id");
    let b = grid.get(0, last).get_tile_id(&model).expect("An id");
    let c = grid.get(last, 0).get_tile_id(&model).expect("An id");
    let d = grid.get(last, last).get_tile_id(&model).expect("An id");
    a * b * c * d
}

// fn calculate_part_2(input: &Day19) -> usize {
//     todo!();
// }

fn solve_grid<'a>(model: &'a Model) -> Option<CandidateGrid<'a>> {
    let grid = CandidateGrid::initial(&model);
    recursive_wave_collapse(grid)
}

fn recursive_wave_collapse<'a>(grid: CandidateGrid<'a>) -> Option<CandidateGrid<'a>> {
    // Check if the grid is broken or solved
    match grid.is_broken_or_solved() {
        Some(true) => return Some(grid),
        Some(false) => return None,
        _ => {}
    };

    // Pick the next cell which we should collapse
    let cell = grid.next_unsolved()?;

    // Recursively try all options for this cell
    for &variant_idx in &cell.candidates {
        let mut next_grid = grid.clone();

        let variant = &grid.model.variants[variant_idx];
        next_grid.place(variant, cell.x, cell.y);

        if let Some(solution) = recursive_wave_collapse(next_grid) {
            return Some(solution);
        }
    }

    None
}

#[derive(Debug, Clone)]
struct CandidateGrid<'a> {
    model: &'a Model<'a>,
    cells: Vec<GridCell>,
}

#[derive(Debug, Clone, Copy)]
struct CellCount(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CellIndex(usize);

impl<'a> CandidateGrid<'a> {
    fn initial(model: &'a Model) -> Self {
        let last = model.inputs.square - 1;
        let mut next_idx = 0;
        let cells = (0..=last)
            .flat_map(|x| (0..=last).map(move |y| (x, y)))
            .map(|(x, y)| {
                let must_connect_top = y > 0;
                let must_connect_bottom = y < last;
                let must_connect_left = x > 0;
                let must_connect_right = x < last;

                let mut tile_set = TileSet::empty();

                let variants = model
                    .variants
                    .iter()
                    .filter(|v| {
                        (!must_connect_top || model.can_connect_on(v, Side::Top))
                            && (!must_connect_bottom || model.can_connect_on(v, Side::Bottom))
                            && (!must_connect_left || model.can_connect_on(v, Side::Left))
                            && (!must_connect_right || model.can_connect_on(v, Side::Right))
                    })
                    .inspect(|v| tile_set.set(v.tile, true))
                    .map(|v| v.id)
                    .collect();

                let idx = CellIndex(next_idx);

                assert_eq!((x, y), Self::index_to_xy(model, idx));
                assert_eq!(idx, Self::xy_to_index(model, x, y));

                next_idx += 1;

                GridCell {
                    x,
                    y,
                    idx,
                    resolved: None,
                    candidates: variants,
                    tile_set,
                }
            });

        let mut result = Self {
            model,
            cells: cells.collect(),
        };

        result.recalculate_candidates(None);
        result
    }

    fn get(&self, x: u8, y: u8) -> &GridCell {
        let idx = Self::xy_to_index(self.model, x, y);
        let cell = &self.cells[idx.0];
        assert!(cell.x == x && cell.y == y);
        cell
    }

    fn index_to_xy(model: &Model, idx: CellIndex) -> (u8, u8) {
        let square = model.inputs.square as usize;
        let idx = idx.0;
        ((idx / square) as u8, (idx % square) as u8)
    }

    fn xy_to_index(model: &Model, x: u8, y: u8) -> CellIndex {
        let square = model.inputs.square as usize;
        CellIndex((x as usize) * square + (y as usize))
    }

    fn is_broken_or_solved(&self) -> Option<bool> {
        for c in &self.cells {
            if c.resolved.is_none() && c.candidates.is_empty() {
                return Some(false);
            }

            if c.resolved.is_none() {
                return None;
            }
        }

        Some(true)
    }

    fn next_unsolved(&self) -> Option<&GridCell> {
        let best = self
            .cells
            .iter()
            .filter_map(|c| {
                if c.resolved.is_none() {
                    Some((c, c.tile_set.count()))
                } else {
                    None
                }
            })
            .min_by_key(|c| c.1)?;
        Some(best.0)
    }

    fn place(&mut self, variant: &Variant, x: u8, y: u8) {
        let target_idx = Self::xy_to_index(self.model, x, y);
        {
            let cell = &mut self.cells[target_idx.0];

            cell.resolved = Some((variant.id, variant.tile));
            cell.candidates.clear();
            cell.tile_set = TileSet::one(variant.tile);
        }
        self.mark_tile_consumed(variant);
        self.recalculate_candidates(Some(target_idx));
    }

    fn recalculate_candidates(&mut self, target_index: Option<CellIndex>) {
        let model = self.model;
        let last = model.inputs.square - 1;

        // Build a work queue of cells we need to check
        // We'll push additional cells on to the queue if we think they need checking or re-checking.
        let mut cell_queue: VecDeque<CellIndex> = if let Some(target_index) = target_index {
            let mut vec = VecDeque::with_capacity(self.cells.len());
            vec.push_front(target_index);
            vec
        } else {
            // No target cell, so check every cell
            (0..self.cells.len()).map(|n| CellIndex(n)).collect()
        };

        fn maybe_revisit(model: &Model, queue: &mut VecDeque<CellIndex>, x: u8, y: u8) {
            let idx = CandidateGrid::xy_to_index(model, x, y);
            if !queue.contains(&idx) {
                // Treating the queue as a stack is much more efficient.
                // It increases the likelihood that the indexes are already queued (and therefore don't need to be requeued)
                // It also reduces the candidates on each specific cell faster, reducing the number of comparisons later on
                queue.push_front(idx);
            }
        }

        while let Some(cell_idx) = cell_queue.pop_front() {
            let (x, y) = Self::index_to_xy(model, cell_idx);

            if x > 0 {
                let x = x - 1;
                let cell_left = Self::xy_to_index(model, x, y);
                assert!(cell_left.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_left, Side::Left) {
                    maybe_revisit(model, &mut cell_queue, x, y);
                }
            }

            if x < last {
                let x = x + 1;
                let cell_right = Self::xy_to_index(model, x, y);
                assert!(cell_right.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_right, Side::Right) {
                    maybe_revisit(model, &mut cell_queue, x, y);
                }
            }

            if y > 0 {
                let y = y - 1;
                let cell_top = Self::xy_to_index(model, x, y);
                assert!(cell_top.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_top, Side::Top) {
                    maybe_revisit(model, &mut cell_queue, x, y);
                }
            }

            if y < last {
                let y = y + 1;
                let cell_bottom = Self::xy_to_index(model, x, y);
                assert!(cell_bottom.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_bottom, Side::Bottom) {
                    maybe_revisit(model, &mut cell_queue, x, y);
                }
            }
        }
    }

    // Find and remove candidates in `other` that can not appear on `side` of `source`
    fn remove_incompatible_candidates(
        &mut self,
        source: CellIndex,
        other: CellIndex,
        side: Side,
    ) -> bool {
        let model = self.model;

        // Borrow both of the cells mutably
        let (other_cell, source_cell) = if source > other {
            let (low, high) = self.cells.split_at_mut(source.0);
            (&mut low[other.0], &mut high[0])
        } else {
            let (low, high) = self.cells.split_at_mut(other.0);
            (&mut high[0], &mut low[source.0])
        };

        // Remember the starting number of candidates. If this changes, then we removed
        // at least one candidate
        let len = other_cell.candidates.len();

        other_cell
            .candidates
            .retain(|&candidate| source_cell.is_allowed_neighbor(model, side, candidate));

        other_cell.candidates.len() != len
    }

    fn mark_tile_consumed(&mut self, variant: &Variant) {
        // This tile can no longer be a candidate for any other cell
        let model = self.model;
        for c in self.cells.iter_mut() {
            c.candidates
                .retain(|&v_id| model.variants[v_id].tile != variant.tile);
            if c.resolved.is_none() {
                c.tile_set.set(variant.tile, false);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct GridCell {
    idx: CellIndex,
    x: u8,
    y: u8,
    resolved: Option<(VariantId, TileIndex)>,
    candidates: Vec<VariantId>,
    tile_set: TileSet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TileSet(BitArr!(for 150, in u64));

impl TileSet {
    fn empty() -> TileSet {
        TileSet(BitArray::<_>::ZERO)
    }

    fn one(tile: TileIndex) -> TileSet {
        let mut set = Self::empty();
        set.0.set(tile.0, true);
        set
    }

    fn is_set(&self, tile: TileIndex) -> bool {
        self.0[tile.0]
    }

    fn set(&mut self, tile: TileIndex, on: bool) {
        self.0.set(tile.0, on)
    }

    fn union(&self, other: &TileSet) -> TileSet {
        TileSet(self.0 | other.0)
    }

    fn intersection(&self, other: &TileSet) -> TileSet {
        TileSet(self.0 & other.0)
    }

    fn difference(&self, other: &TileSet) -> TileSet {
        let i = self.intersection(other);
        TileSet(self.0 & (!i.0))
    }

    fn count(&self) -> usize {
        self.0.count_ones()
    }

    fn tiles<'a>(&'a self) -> impl Iterator<Item = TileIndex> + 'a {
        self.0.iter_ones().map(|t| TileIndex(t))
    }
}

impl GridCell {
    fn get_tile_id(&self, model: &Model) -> Option<i64> {
        let (_, tile_idx) = self.resolved?;
        let tile = model.inputs.tiles.get(tile_idx.0)?;
        Some(tile.id)
    }

    fn is_allowed_neighbor(&self, model: &Model, side: Side, candidate: VariantId) -> bool {
        match &self.resolved {
            Some((variant, _)) => model
                .adjacent
                .get(&(*variant, side))
                .map(|allowable| allowable.contains(&candidate))
                .unwrap_or(false),
            None => self
                .candidates
                .iter()
                .filter_map(|&variant| model.adjacent.get(&(variant, side)))
                .any(|allowable| allowable.contains(&candidate)),
        }
    }

    fn get_side_of(&self, other: &GridCell) -> Option<Side> {
        let (a_x, a_y, b_x, b_y) = (self.x as i16, self.y as i16, other.x as i16, other.y as i16);
        match (a_x - b_x, a_y - b_y) {
            // X increases left to right, so if a > b then b is to the left if a
            (1, 0) => Some(Side::Left),
            (-1, 0) => Some(Side::Right),
            // Y increases top to bottom, so if a > b then b is above a
            (0, 1) => Some(Side::Top),
            (0, -1) => Some(Side::Bottom),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Day20 {
    tiles: Vec<Tile>,
    square: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TileIndex(usize);

#[derive(Debug)]
struct Tile {
    idx: TileIndex,
    id: i64,
    map: Bitmap,
}

#[derive(Debug, Clone, Copy)]
enum Flip {
    None,
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Side {
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
struct Edges {
    top: Edge,
    bottom: Edge,
    left: Edge,
    right: Edge,
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

struct EdgesIterator<'a> {
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

type VariantId = usize;

#[derive(Debug)]
struct Variant {
    id: VariantId,
    tile: TileIndex,
    flip: Flip,
    rotation: Rotation,
    map: Bitmap,
    edges: Edges,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct TileScore(usize);

type AdjacentMap = HashMap<(VariantId, Side), Vec<VariantId>>;

#[derive(Debug)]
struct Model<'a> {
    inputs: &'a Day20,
    variants: Vec<Variant>,
    adjacent: AdjacentMap,
}

impl<'a> Model<'a> {
    fn build(inputs: &'a Day20) -> Self {
        let variants: Vec<Variant> = inputs
            .tiles
            .iter()
            .flat_map(|t| Model::make_variants(t))
            .enumerate()
            .map(|(idx, mut v)| {
                v.id = idx;
                v
            })
            .collect();
        let adjacent = Model::calculate_adjacents(&variants);
        Self {
            inputs,
            variants,
            adjacent,
        }
    }

    const ALL_FLIPS: [Flip; 4] = [Flip::None, Flip::Horizontal, Flip::Vertical, Flip::Both];
    const ALL_ROTATIONS: [Rotation; 4] =
        [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];

    fn make_variants(tile: &'a Tile) -> impl Iterator<Item = Variant> + 'a {
        Model::ALL_FLIPS
            .iter()
            .cloned()
            .cartesian_product(Model::ALL_ROTATIONS.iter().cloned())
            .map(move |(f, r)| {
                let map = tile.map.flip(f).rotate(r);
                Variant {
                    id: 0, // We'll fix this later
                    tile: tile.idx,
                    flip: f,
                    rotation: r,
                    map,
                    edges: map.make_edges(),
                }
            })
            .unique_by(|v| v.map)
    }

    fn calculate_adjacents(variants: &[Variant]) -> AdjacentMap {
        let grouped = variants
            .iter()
            .flat_map(|a| variants.iter().map(move |b| (a, b)))
            .filter(|(a, b)| a.tile != b.tile)
            .flat_map(|(a, b)| Model::get_adjoining_edges(a, b).map(move |e| (a.id, e.side, b.id)))
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

    fn can_connect_on(&self, variant: &Variant, side: Side) -> bool {
        match self.adjacent.get(&(variant.id, side)) {
            Some(adj) => !adj.is_empty(),
            None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bitmap {
    pixels: u128,
}

impl Bitmap {
    fn empty() -> Bitmap {
        Bitmap { pixels: 0 }
    }

    fn flip(&self, flip: Flip) -> Bitmap {
        let mut result = Bitmap::empty();
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

    fn rotate(&self, rotation: Rotation) -> Bitmap {
        let mut result = Bitmap::empty();
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

    fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(10 * (10 + 2));
        for y in 0..10 {
            for x in 0..10 {
                let sym = if self.is_set(x, y) { '#' } else { '.' };
                buffer.push(sym);
            }
            buffer.push_str("\r\n");
        }

        buffer
    }

    fn is_set(&self, x: u8, y: u8) -> bool {
        (Bitmap::to_mask(x, y) & self.pixels) > 0
    }

    fn set(&mut self, x: u8, y: u8, on: bool) {
        let mask = Bitmap::to_mask(x, y);
        if on {
            self.pixels |= mask;
        } else {
            self.pixels &= !mask;
        }
    }

    fn make_edges(&self) -> Edges {
        let mut top = Edge::new(Side::Top);
        let mut bottom = Edge::new(Side::Bottom);
        let mut left = Edge::new(Side::Left);
        let mut right = Edge::new(Side::Right);
        for i in 0..10 {
            top.set(i, self.is_set(i, 0));
            bottom.set(i, self.is_set(i, 9));
            left.set(i, self.is_set(0, i));
            right.set(i, self.is_set(9, i));
        }
        Edges {
            top,
            bottom,
            left,
            right,
        }
    }

    fn to_mask(x: u8, y: u8) -> u128 {
        1 << Bitmap::to_idx(x, y)
    }

    fn to_idx(x: u8, y: u8) -> u8 {
        assert!(x <= 9 && y <= 9);
        ((9 - y) * 10) + (9 - x)
    }
}

#[derive(Debug)]
struct Edge {
    pixels: u16,
    side: Side,
}

impl Edge {
    fn new(side: Side) -> Self {
        Edge { pixels: 0, side }
    }

    fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(10);
        for w in 0..10 {
            let sym = if self.is_set(w) { '#' } else { '.' };
            buffer.push(sym);
        }

        buffer
    }

    fn is_set(&self, w: u8) -> bool {
        (self.pixels & Edge::to_mask(w)) > 0
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

fn inputs() -> Day20 {
    let text = crate::util::read_file("inputs/day20.txt");
    parse_input(&text)
}

fn parse_input(text: &str) -> Day20 {
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
                map: Bitmap::empty(),
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

    Day20 { tiles, square }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###

            Tile 1951:
            #.##...##.
            #.####...#
            .....#..##
            #...######
            .##.#....#
            .###.#####
            ###.##.##.
            .###....#.
            ..#.#..#.#
            #...##.#..

            Tile 1171:
            ####...##.
            #..##.#..#
            ##.#..#.#.
            .###.####.
            ..###.####
            .##....##.
            .#...####.
            #.##.####.
            ####..#...
            .....##...

            Tile 1427:
            ###.##.#..
            .#..#.##..
            .#.##.#..#
            #.#.#.##.#
            ....#...##
            ...##..##.
            ...#.#####
            .#.####.#.
            ..#..###.#
            ..##.#..#.

            Tile 1489:
            ##.#.#....
            ..##...#..
            .##..##...
            ..#...#...
            #####...#.
            #..#.#.#.#
            ...#.#.#..
            ##.#...##.
            ..##.##.##
            ###.##.#..

            Tile 2473:
            #....####.
            #..#.##...
            #.##..#...
            ######.#.#
            .#...#.#.#
            .#########
            .###.#..#.
            ########.#
            ##...##.#.
            ..###.#.#.

            Tile 2971:
            ..#.#....#
            #...###...
            #.#.###...
            ##.##..#..
            .#####..##
            .#..####.#
            #..#.#..#.
            ..####.###
            ..#.#.###.
            ...#.#.#.#

            Tile 2729:
            ...#.#.#.#
            ####.#....
            ..#.#.....
            ....#..#.#
            .##..##.#.
            .#.####...
            ####.#.#..
            ##.####...
            ##..#.##..
            #.##...##.

            Tile 3079:
            #.#.#####.
            .#..######
            ..#.......
            ######....
            ####.#..#.
            .#...#.##.
            #.#####.##
            ..#.###...
            ..#.......
            ..#.###...
        ";

        let input = parse_input(text);
        assert_eq!(20899048083289, calculate_part_1(&input));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(23386616781851, calculate_part_1(&inputs));
        // assert_eq!(253, calculate_part_2(&mut inputs));
    }
}
