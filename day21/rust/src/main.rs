fn part1() {
    let mut pos = [4, 1];
    let mut score = [0, 0];
    let (mut dice, mut roll) = (1, 0);
    let mut turn = 0;

    while score[0] < 1000 && score[1] < 1000 {
        let mut sum = 0;
        for _ in 0..3 {
            sum += dice;
            dice += 1;
            if dice > 100 {
                dice = 1;
            }
        }
        roll += 3;
        pos[turn] = (pos[turn] - 1 + sum) % 10 + 1;
        score[turn] += pos[turn];
        turn ^= 1;
    }
    println!("{} * {} = {}", roll, score[turn], roll * score[turn]);
}

fn part2() {
    /* 3면체 주사위를 3번 굴림
    합3 = 1가지 (1, 1, 1)
    합4 = 3가지 (1, 1, 2)
    합5 = 6가지 (1, 1, 3), (1, 2, 2)
    합6 = 7가지 (1, 2, 3), (2, 2, 2)
    합7 = 6가지 (1, 3, 3), (2, 2, 3)
    합8 = 3가지 (2, 3, 3)
    합9 = 1가지 (3, 3, 3)
    */
    let cases = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    // pos, pos, score, score, turn, #case
    let mut states = vec![[4, 1, 0, 0, 0, 1]];
    let mut win: [u64; 2] = [0, 0];

    loop {
        if let Some(state) = states.pop() {
            let turn = state[4] as usize;
            let pos = state[turn];
            let score = state[2 + turn];
            let case_sum = state[5];
            for (sum, case) in cases {
                let pos = (pos - 1 + sum) % 10 + 1;
                let score = score + pos;
                let case = case_sum * case;
                if score >= 21 {
                    win[turn] += case;
                } else {
                    match turn {
                        0 => states.push([pos, state[1], score, state[3], (turn ^ 1) as _, case]),
                        1 => states.push([state[0], pos, state[2], score, (turn ^ 1) as _, case]),
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            break;
        }
    }
    println!("{}, {}, {}", win[0] < win[1], win[0], win[1]);
}

fn main() {
    part1();
    part2();
}
