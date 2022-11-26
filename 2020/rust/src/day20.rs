mod bitmap;
mod input;
mod monster;
mod wave;

use self::{bitmap::Bitmap, monster::ResolvedImage, wave::solve_grid};
use input::*;

pub fn run_day_twenty() {
    let inputs = inputs();
    let (part_1, part_2) = calculate_both_parts(&inputs);
    println!("Day 20, Part 1: {}", part_1);
    println!("Day 20, Part 2: {}", part_2);
}

fn calculate_both_parts(input: &Inputs) -> (i64, i64) {
    let expanded_input = ExpandedInput::build(input);
    let grid = solve_grid(&expanded_input).expect("A solution");
    let part_1 = grid.corner_product();
    let resolved: ResolvedImage = grid.into();
    eprintln!("Resolved: \r\n{}", resolved.to_string());
    (part_1, 0)
}

// fn calculate_part_2(input: &Day19) -> usize {
//     todo!();
// }

fn inputs() -> Inputs {
    let text = crate::util::read_file("inputs/day20.txt");
    Inputs::parse(&text)
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

        let inputs = Inputs::parse(text);
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(20899048083289, part_1);
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(23386616781851, part_1);
        // assert_eq!(253, calculate_part_2(&mut inputs));
    }
}
