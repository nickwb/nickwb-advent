mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_use]
extern crate lazy_static;

fn main() {
    day1::run_day_one();
    day2::run_day_two();
    day3::run_day_three();
    day4::run_day_four();
    day5::run_day_five();
    day6::run_day_six();
    day7::run_day_seven();
    day8::run_day_eight();
    day9::run_day_nine();
}
