use std::collections::{HashSet, VecDeque};

pub fn run_day_twenty_two() {
    let mut inputs = inputs();
    let (part_1, part_2) = calculate_both_parts(&mut inputs);
    println!("Day 22, Part 1: {}", part_1);
    println!("Day 22, Part 2: {}", part_2);
}

fn calculate_both_parts(inputs: &mut Inputs) -> (usize, usize) {
    let mut first = inputs.clone();
    first.play_simple();
    let part_1 = first.get_score();

    inputs.play_recursive();
    let part_2 = inputs.get_score();
    (part_1, part_2)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Inputs {
    player_one: VecDeque<usize>,
    player_two: VecDeque<usize>,
}

type Snapshot = Vec<u8>;

impl Inputs {
    fn play_simple(&mut self) {
        while !self.player_one.is_empty() && !self.player_two.is_empty() {
            let one = self.player_one.pop_front().unwrap();
            let two = self.player_two.pop_front().unwrap();

            if one > two {
                self.player_one.push_back(one);
                self.player_one.push_back(two);
            } else if two > one {
                self.player_two.push_back(two);
                self.player_two.push_back(one);
            } else {
                panic!("No winner");
            }
        }
    }

    fn play_recursive(&mut self) -> usize {
        let mut seen_states: HashSet<Snapshot> = HashSet::new();
        while !self.player_one.is_empty() && !self.player_two.is_empty() {
            let snapshot = self.to_snapshot();
            if seen_states.contains(&snapshot) {
                return 1;
            }
            seen_states.insert(snapshot);
            let one = self.player_one.pop_front().unwrap();
            let two = self.player_two.pop_front().unwrap();
            let winner = if (one > self.player_one.len()) || (two > self.player_two.len()) {
                if one > two {
                    1
                } else if two > one {
                    2
                } else {
                    panic!("Unclear winner");
                }
            } else {
                let mut sub_game = self.clone();
                while sub_game.player_one.len() > one {
                    sub_game.player_one.pop_back();
                }
                while sub_game.player_two.len() > two {
                    sub_game.player_two.pop_back();
                }
                sub_game.play_recursive()
            };
            if winner == 1 {
                self.player_one.push_back(one);
                self.player_one.push_back(two);
            } else if winner == 2 {
                self.player_two.push_back(two);
                self.player_two.push_back(one);
            } else {
                panic!("I'm confused about the winner");
            }
        }

        if self.player_one.is_empty() {
            2
        } else {
            1
        }
    }

    fn get_score(&self) -> usize {
        let winner = if self.player_one.is_empty() {
            &self.player_two
        } else {
            &self.player_one
        };

        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i + 1) * x)
            .sum()
    }

    fn to_snapshot(&self) -> Snapshot {
        let len = self.player_one.len() + self.player_two.len() + 1;
        let mut buf = Vec::with_capacity(len);
        buf.push(self.player_one.len() as u8);
        buf.extend(self.player_one.iter().map(|&x| x as u8));
        buf.extend(self.player_two.iter().map(|&x| x as u8));
        buf
    }

    fn parse(text: &str) -> Self {
        let lines = text.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let mut player = 0;
        let mut player_one: VecDeque<usize> = VecDeque::new();
        let mut player_two: VecDeque<usize> = VecDeque::new();
        for l in lines {
            if l == "Player 1:" {
                player = 1;
                continue;
            } else if l == "Player 2:" {
                player = 2;
                continue;
            } else if player == 0 {
                panic!("No player selected");
            }

            let num: usize = l
                .parse()
                .expect(&format!("A valid numbered card, not: {l}"));
            if player == 1 {
                player_one.push_back(num);
            } else {
                player_two.push_back(num);
            }
        }

        Self {
            player_one,
            player_two,
        }
    }
}

fn inputs() -> Inputs {
    let text = crate::util::read_file("inputs/day22.txt");
    Inputs::parse(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            Player 1:
            9
            2
            6
            3
            1
            
            Player 2:
            5
            8
            4
            7
            10
        ";

        let mut inputs = Inputs::parse(text);
        let (part_1, part_2) = calculate_both_parts(&mut inputs);
        assert_eq!(306, part_1);
        assert_eq!(291, part_2);
    }

    #[test]
    #[cfg(feature = "slow_problems")]
    fn actual_inputs() {
        let mut inputs = inputs();
        let (part_1, part_2) = calculate_both_parts(&mut inputs);
        assert_eq!(35562, part_1);
        assert_eq!(34424, part_2);
    }
}
