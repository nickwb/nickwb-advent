use super::input::{ExpandedInput, TileIndex, Variant, VariantId};
use crate::day20::input::Side;
use bitvec::{prelude::BitArray, BitArr};
use std::collections::VecDeque;

pub fn solve_grid<'a>(expanded_input: &'a ExpandedInput) -> Option<CandidateGrid<'a>> {
    let grid = CandidateGrid::initial(&expanded_input);
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

        let variant = &grid.expanded_input.variants[variant_idx.0];
        next_grid.place(variant, cell.x, cell.y);

        if let Some(solution) = recursive_wave_collapse(next_grid) {
            return Some(solution);
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct CandidateGrid<'a> {
    pub expanded_input: &'a ExpandedInput<'a>,
    cells: Vec<GridCell>,
}

impl<'a> CandidateGrid<'a> {
    fn initial(expanded_input: &'a ExpandedInput) -> Self {
        let last = expanded_input.inputs.last_idx;
        let mut next_idx = 0;
        let cells = (0..=last)
            .flat_map(|x| (0..=last).map(move |y| (x, y)))
            .map(|(x, y)| {
                let must_connect_top = y > 0;
                let must_connect_bottom = y < last;
                let must_connect_left = x > 0;
                let must_connect_right = x < last;

                let mut tile_set = TileSet::empty();

                let variants = expanded_input
                    .variants
                    .iter()
                    .filter(|v| {
                        (!must_connect_top || expanded_input.can_connect_on(v, Side::Top))
                            && (!must_connect_bottom
                                || expanded_input.can_connect_on(v, Side::Bottom))
                            && (!must_connect_left || expanded_input.can_connect_on(v, Side::Left))
                            && (!must_connect_right
                                || expanded_input.can_connect_on(v, Side::Right))
                    })
                    .inspect(|v| tile_set.set(v.tile, true))
                    .map(|v| v.id)
                    .collect();

                let idx = CellIndex(next_idx);

                // assert_eq!((x, y), Self::index_to_xy(expanded_input, idx));
                // assert_eq!(idx, Self::xy_to_index(expanded_input, x, y));

                next_idx += 1;

                GridCell {
                    x,
                    y,
                    _idx: idx,
                    resolved: None,
                    candidates: variants,
                    tile_set,
                }
            });

        let mut result = Self {
            expanded_input,
            cells: cells.collect(),
        };

        result.recalculate_candidates(None);
        result
    }

    fn get(&self, x: u8, y: u8) -> &GridCell {
        let idx = Self::xy_to_index(self.expanded_input, x, y);
        let cell = &self.cells[idx.0];
        //assert!(cell.x == x && cell.y == y);
        cell
    }

    fn index_to_xy(expanded_input: &ExpandedInput, idx: CellIndex) -> (u8, u8) {
        let square = expanded_input.inputs.square as usize;
        let idx = idx.0;
        ((idx / square) as u8, (idx % square) as u8)
    }

    fn xy_to_index(expanded_input: &ExpandedInput, x: u8, y: u8) -> CellIndex {
        let square = expanded_input.inputs.square as usize;
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
        let target_idx = Self::xy_to_index(self.expanded_input, x, y);
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
        let expanded_input = self.expanded_input;
        let last = expanded_input.inputs.last_idx;

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

        fn maybe_revisit(
            expanded_input: &ExpandedInput,
            queue: &mut VecDeque<CellIndex>,
            x: u8,
            y: u8,
        ) {
            let idx = CandidateGrid::xy_to_index(expanded_input, x, y);
            if !queue.contains(&idx) {
                // Treating the queue as a stack is much more efficient.
                // It increases the likelihood that the indexes are already queued (and therefore don't need to be requeued)
                // It also reduces the candidates on each specific cell faster, reducing the number of comparisons later on
                queue.push_front(idx);
            }
        }

        while let Some(cell_idx) = cell_queue.pop_front() {
            let (x, y) = Self::index_to_xy(expanded_input, cell_idx);

            if x > 0 {
                let x = x - 1;
                let cell_left = Self::xy_to_index(expanded_input, x, y);
                //assert!(cell_left.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_left, Side::Left) {
                    maybe_revisit(expanded_input, &mut cell_queue, x, y);
                }
            }

            if x < last {
                let x = x + 1;
                let cell_right = Self::xy_to_index(expanded_input, x, y);
                //assert!(cell_right.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_right, Side::Right) {
                    maybe_revisit(expanded_input, &mut cell_queue, x, y);
                }
            }

            if y > 0 {
                let y = y - 1;
                let cell_top = Self::xy_to_index(expanded_input, x, y);
                //assert!(cell_top.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_top, Side::Top) {
                    maybe_revisit(expanded_input, &mut cell_queue, x, y);
                }
            }

            if y < last {
                let y = y + 1;
                let cell_bottom = Self::xy_to_index(expanded_input, x, y);
                //assert!(cell_bottom.0 < self.cells.len());
                if self.remove_incompatible_candidates(cell_idx, cell_bottom, Side::Bottom) {
                    maybe_revisit(expanded_input, &mut cell_queue, x, y);
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
        let expanded_input = self.expanded_input;

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
            .retain(|&candidate| source_cell.is_allowed_neighbor(expanded_input, side, candidate));

        other_cell.candidates.len() != len
    }

    fn mark_tile_consumed(&mut self, variant: &Variant) {
        // This tile can no longer be a candidate for any other cell
        let expanded_input = self.expanded_input;
        for c in self.cells.iter_mut() {
            c.candidates
                .retain(|&v_id| expanded_input.variants[v_id.0].tile != variant.tile);
            if c.resolved.is_none() {
                c.tile_set.set(variant.tile, false);
            }
        }
    }

    pub fn corner_product(&self) -> i64 {
        let last = self.expanded_input.inputs.last_idx;
        let a = self
            .get(0, 0)
            .get_tile_id(&self.expanded_input)
            .expect("Expected to resolve Tile Id at 0,0");
        let b = self
            .get(0, last)
            .get_tile_id(&self.expanded_input)
            .expect("Expected to resolve Tile Id at 0,n");
        let c = self
            .get(last, 0)
            .get_tile_id(&self.expanded_input)
            .expect("Expected to resolve Tile Id at n,0");
        let d = self
            .get(last, last)
            .get_tile_id(&self.expanded_input)
            .expect("Expected to resolve Tile Id at n,n");
        a * b * c * d
    }

    pub fn variant_at(&self, x: u8, y: u8) -> &Variant {
        let cell = self.get(x, y);
        if let Some((v, _)) = cell.resolved {
            &self.expanded_input.variants[v.0]
        } else {
            panic!("Unresolved cell");
        }
    }
}

#[derive(Debug, Clone)]
struct GridCell {
    _idx: CellIndex,
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

    fn set(&mut self, tile: TileIndex, on: bool) {
        self.0.set(tile.0, on)
    }

    fn count(&self) -> usize {
        self.0.count_ones()
    }
}

impl GridCell {
    fn get_tile_id(&self, expanded_input: &ExpandedInput) -> Option<i64> {
        let (_, tile_idx) = self.resolved?;
        let tile = expanded_input.inputs.tiles.get(tile_idx.0)?;
        Some(tile.id)
    }

    fn is_allowed_neighbor(
        &self,
        expanded_input: &ExpandedInput,
        side: Side,
        candidate: VariantId,
    ) -> bool {
        match &self.resolved {
            Some((variant, _)) => expanded_input
                .adjacent
                .get(&(*variant, side))
                .map(|allowable| allowable.contains(&candidate))
                .unwrap_or(false),
            None => self
                .candidates
                .iter()
                .filter_map(|&variant| expanded_input.adjacent.get(&(variant, side)))
                .any(|allowable| allowable.contains(&candidate)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CellCount(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CellIndex(usize);
