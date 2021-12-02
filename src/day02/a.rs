pub fn main() {
    let result = super::DATA
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, units)| (dir, units.parse::<i32>().unwrap()))
        .fold((0, 0), |(pos, depth), (dir, units)| match dir {
            "forward" => (pos + units, depth),
            "down" => (pos, depth + units),
            "up" => (pos, depth - units),
            _ => unreachable!(),
        });

    println!("{} = {}", module_path!(), result.0 * result.1);
}
