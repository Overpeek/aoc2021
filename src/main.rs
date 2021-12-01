use std::env::args;

fn main() {
    let mut n: Option<isize> = None;
    for arg in args() {
        if let Ok(arg) = arg.parse() {
            n = Some(arg);
        }
    }

    if let Some(n) = n {
        if let Some(f) = ALL.get((n - 1) as usize) {
            f();
        } else {
            println!("Index: {} out of bounds: [1, 50]", n);
        }
    } else {
        for f in ALL {
            f();
        }
    }
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

const I: &str = "<incomplete>";

const ALL: [&dyn Fn(); 50] = [
    &day01::a::main as &dyn Fn(),
    &day01::b::main as _,
    &day02::a::main as _,
    &day02::b::main as _,
    &day03::a::main as _,
    &day03::b::main as _,
    &day04::a::main as _,
    &day04::b::main as _,
    &day05::a::main as _,
    &day05::b::main as _,
    &day06::a::main as _,
    &day06::b::main as _,
    &day07::a::main as _,
    &day07::b::main as _,
    &day08::a::main as _,
    &day08::b::main as _,
    &day09::a::main as _,
    &day09::b::main as _,
    &day10::a::main as _,
    &day10::b::main as _,
    &day11::a::main as _,
    &day11::b::main as _,
    &day12::a::main as _,
    &day12::b::main as _,
    &day13::a::main as _,
    &day13::b::main as _,
    &day14::a::main as _,
    &day14::b::main as _,
    &day15::a::main as _,
    &day15::b::main as _,
    &day16::a::main as _,
    &day16::b::main as _,
    &day17::a::main as _,
    &day17::b::main as _,
    &day18::a::main as _,
    &day18::b::main as _,
    &day19::a::main as _,
    &day19::b::main as _,
    &day20::a::main as _,
    &day20::b::main as _,
    &day21::a::main as _,
    &day21::b::main as _,
    &day22::a::main as _,
    &day22::b::main as _,
    &day23::a::main as _,
    &day23::b::main as _,
    &day24::a::main as _,
    &day24::b::main as _,
    &day25::a::main as _,
    &day25::b::main as _,
];
