pub fn main() {
    let bitlen = super::DATA.lines().next().unwrap().len();
    let nums: Vec<_> = super::DATA
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();
    let numc = nums.len();

    let gamma = (0..bitlen).fold(0, |acc, i| {
        let common = numc > 2 * nums.iter().filter(|num| (1 << i) & *num == 0).count();
        acc + ((common as usize) << i)
    });

    let epsilon = gamma ^ (2_usize.pow(bitlen as u32) - 1);

    println!("{} = {}", module_path!(), gamma * epsilon);
}
