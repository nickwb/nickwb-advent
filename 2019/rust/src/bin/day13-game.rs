use std::io::Write;

use nickwb_advent_2019::day13::GameState;

fn main() {
    let mut state = GameState::new();
    let mut move_input = String::with_capacity(5);
    let mut is_cheating = false;

    while !state.run_one_cycle() {
        print!("Move: ");
        std::io::stdout().lock().flush().unwrap();
        std::io::stdin().read_line(&mut move_input).unwrap();
        match move_input.trim() {
            "a" => state.buffer_left(),
            "d" => state.buffer_right(),
            "x" => {
                is_cheating = !is_cheating;
                state.buffer_optimal();
            }
            _ => {
                if is_cheating {
                    state.buffer_optimal()
                } else {
                    state.buffer_neutral()
                }
            }
        }
        move_input.clear();
        println!("");
    }
}
