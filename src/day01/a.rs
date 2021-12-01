pub fn main() {
    let nums: Vec<u16> = super::DATA
        .split('\n')
        .map(|num| num.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let result = nums
        .iter()
        .zip(nums.iter().skip(1))
        .fold(0, |acc, (last, this)| acc + (this > last) as i32);

    println!("{} = {}", module_path!(), result);
}
