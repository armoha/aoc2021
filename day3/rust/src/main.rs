const DIGIT: usize = 12;
const COUNT: usize = 1000;

fn read() -> Vec<u32> {
    include_str!("../../input.txt")
        .lines()
        .map(|b| u32::from_str_radix(b, 2).unwrap())
        .collect()
}

fn part1(nums: &[u32]) -> u32 {
    let mut digits = [0; 12];
    for num in nums {
        for bit in 0..DIGIT {
            let b = 1 << bit as u32;
            if num & b == b {
                digits[bit] += 1;
            }
        }
    }
    println!("{:?}", digits);
    let x = digits
        .into_iter()
        .enumerate()
        .fold(0, |acc, v| acc + (((v.1 >= COUNT / 2) as u32) << v.0));
    x * (!x & ((1 << DIGIT) - 1))
}

fn part2(nums: &mut Vec<u32>) -> u32 {
    let oxy = (0..DIGIT)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.retain(|n| (*n & 1 << i > 0) == one);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..DIGIT)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.retain(|n| (*n & 1 << i > 0) != one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    oxy * co2
}

fn main() {
    let mut input = read();
    println!(
        "Day 3 Part 1: {}, Part 2: {}",
        part1(&input),
        part2(&mut input)
    );
}
