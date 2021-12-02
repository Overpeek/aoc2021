pub fn main() {
    let nums: Vec<u16> = super::DATA
        .lines()
        .map(|num| num.parse().unwrap())
        .collect();

    let result = nums.windows(2).filter(|w| w[0] < w[1]).count();

    println!("{} = {}", module_path!(), result);
}
