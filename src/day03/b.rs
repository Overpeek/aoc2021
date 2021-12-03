// lhs = o2
// rhs = co2
fn run(nums: &[usize], bit_i: usize) -> (Vec<usize>, Vec<usize>) {
    // lhs = 0 bit at bit_i
    // rhs = 1 bit at bit_i
    let (lhs, rhs) = nums
        .iter()
        .partition::<Vec<_>, _>(|num| (1 << bit_i) & *num == 0);

    match lhs.len().cmp(&rhs.len()) {
        std::cmp::Ordering::Equal => (rhs, lhs),
        std::cmp::Ordering::Less => (rhs, lhs),
        std::cmp::Ordering::Greater => (lhs, rhs),
    }
}

pub fn main() {
    let bitlen = super::DATA.lines().next().unwrap().len();
    let nums: Vec<_> = super::DATA
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let (mut o2, mut co2) = run(&nums, bitlen - 1);
    for i in (0..bitlen - 1).rev() {
        if o2.len() != 1 {
            o2 = run(&o2, i).0;
        }
        if co2.len() != 1 {
            co2 = run(&co2, i).1;
        }
    }
    let (o2, co2) = (*o2.first().unwrap(), *co2.first().unwrap());

    println!("{} = {}", module_path!(), o2 * co2);
}
