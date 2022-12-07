mod bitmap;
mod input;
mod monster;
mod wave;

use self::{
    monster::{subtract_monsters, ResolvedImage},
    wave::solve_grid,
};
use input::*;

pub fn run_day_twenty() {
    #[cfg(feature = "slow_problems")]
    {
        let inputs = inputs();
        let (part_1, part_2) = calculate_both_parts(&inputs);

        println!("Day 20, Part 1: {}", part_1);
        println!("Day 20, Part 2: {}", part_2);
    }

    #[cfg(not(feature = "slow_problems"))]
    {
        println!("Day 20, Part 1: SKIPPED");
        println!("Day 20, Part 2: SKIPPED");
    }
}

fn calculate_both_parts(input: &Inputs) -> (i64, i64) {
    let expanded_input = ExpandedInput::build(input);
    let grid =
        solve_grid(&expanded_input).expect("Expected the input to be solvable, but it was not");
    let part_1 = grid.corner_product();
    let resolved: ResolvedImage = grid.into();
    let monsters_removed = subtract_monsters(&resolved)
        .expect("Expected one or more monsters to be found, but they were not");
    let part_2 = monsters_removed.roughness();
    (part_1, part_2)
}

fn inputs() -> Inputs {
    let text = crate::util::read_file("inputs/day20.txt");
    Inputs::parse(&text)
}

#[cfg(test)]
#[cfg(feature = "slow_problems")]
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

        let inputs = Inputs::parse(text);
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(20899048083289, part_1);
        assert_eq!(273, part_2);
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(23386616781851, part_1);
        assert_eq!(2376, part_2);
    }
}
