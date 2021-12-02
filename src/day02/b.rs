pub fn main() {
    let result = super::DATA
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, units)| (dir, units.parse::<i32>().unwrap()))
        .fold((0, 0, 0), |(pos, depth, aim), (dir, units)| match dir {
            "forward" => (pos + units, depth + aim * units, aim),
            "down" => (pos, depth, aim + units),
            "up" => (pos, depth, aim - units),
            _ => unreachable!(),
        });

    println!("{} = {}", module_path!(), result.0 * result.1);
}
