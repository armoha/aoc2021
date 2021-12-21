enum Move {
    Down,
    Up,
    Forward,
}

fn read() -> Vec<(Move, i32)> {
    include_str!("../../input.txt")
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(mov, amount)| match (mov, amount.parse().unwrap()) {
            ("down", amount) => (Move::Down, amount),
            ("up", amount) => (Move::Up, amount),
            ("forward", amount) => (Move::Forward, amount),
            _ => unreachable!(),
        })
        .collect()
}

fn part1(nums: &[(Move, i32)]) -> i32 {
    let (mut h, mut v) = (0, 0);
    for (mov, amount) in nums {
        match mov {
            Move::Down => v += amount,
            Move::Up => v -= amount,
            Move::Forward => h += amount,
        }
    }
    h * v
}

fn part2(nums: &[(Move, i32)]) -> i32 {
    let (mut h, mut v, mut aim) = (0, 0, 0);
    for (mov, amount) in nums {
        match mov {
            Move::Down => aim += amount,
            Move::Up => aim -= amount,
            Move::Forward => {
                h += amount;
                v += aim * amount;
            }
        }
    }
    h * v
}

fn main() {
    let input = read();
    println!("Day 2 Part 1: {}, Part 2: {}", part1(&input), part2(&input),);
}
