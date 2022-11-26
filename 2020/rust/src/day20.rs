mod input;
mod monster;
mod wave;

use self::wave::solve_grid;
use input::*;

pub fn run_day_twenty() {
    let inputs = inputs();
    println!("Day 20, Part 1: {}", calculate_part_1(&inputs));
    // println!("Day 20, Part 2: {}", calculate_part_2(&mut inputs));
}

fn calculate_part_1(input: &Inputs) -> i64 {
    let expanded_input = ExpandedInput::build(input);
    let grid = solve_grid(&expanded_input).expect("A solution");
    grid.corner_product()
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

        let input = Inputs::parse(text);
        assert_eq!(20899048083289, calculate_part_1(&input));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(23386616781851, calculate_part_1(&inputs));
        // assert_eq!(253, calculate_part_2(&mut inputs));
    }
}
