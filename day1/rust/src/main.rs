fn read() -> Vec<u16> {
    include_str!("../../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn zipcmp(arr: &[u16], start: usize) -> u32 {
    let mut ret = 0;
    for (p, q) in arr[..(arr.len() - start)]
        .into_iter()
        .zip(arr[start..].into_iter())
    {
        if p < q {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let input = read();
    println!(
        "Part 1: {}, Part 2: {}",
        zipcmp(&input, 1),
        zipcmp(&input, 3)
    );
}
