pub fn main() {
    let bitlen = super::DATA.lines().next().unwrap().len();
    let nums: Vec<u16> = super::DATA
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();
    let numc = nums.len();

    let result = nums.into_iter().fold(vec![0; bitlen], |mut acc, num| {
        acc.iter_mut()
            .enumerate()
            .for_each(|(i, acc)| *acc += ((1 << i) & num) >> i);
        acc
    });

    let gamma = result.iter().enumerate().fold(0, |acc, (i, bitc)| {
        acc + (((numc < 2 * *bitc as usize) as u32) << i)
    });

    let epsilon = gamma ^ (2_u32.pow(bitlen as u32) - 1);

    println!("{} = {}", module_path!(), gamma * epsilon);
}
