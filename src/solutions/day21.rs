pub fn part1(text: &str) {
    let mut lines = text.lines();
    let mut player1 = (lines.next().unwrap()[28..].parse::<usize>().unwrap(), 0);
    let mut player2 = (lines.next().unwrap()[28..].parse::<usize>().unwrap(), 0);
    let (mut die, mut rolls) = (0, 0);
    println!(
        "part1: {}",
        loop {
            play(&mut player1, &mut die, &mut rolls);
            if player1.1 >= 1000 {
                break rolls * player2.1;
            }
            play(&mut player2, &mut die, &mut rolls);
            if player2.1 >= 1000 {
                break rolls * player1.1;
            }
        }
    );
}

fn play((pos, p): &mut (usize, usize), die: &mut usize, rolls: &mut usize) {
    *pos += roll_dice(die);
    *rolls += 3;
    *pos = ((*pos - 1) % 10) + 1;
    *p += *pos;
}

fn roll_dice(dice: &mut usize) -> usize {
    let mut result = 0;
    for _ in 0..3 {
        *dice += 1;
        if *dice > 100 {
            *dice = 1;
        }
        result += *dice;
    }
    result
}

pub fn part2(text: &str) {
    let mut lines = text.lines();
    let pos1 = lines.next().unwrap()[28..].parse::<usize>().unwrap();
    let pos2 = lines.next().unwrap()[28..].parse::<usize>().unwrap();
    let mut game_state = vec![vec![vec![vec![None; 21]; 21]; 10]; 10];
    let (w1, w2) = quantum_game(&mut game_state, 0, 0, pos1 - 1, pos2 - 1);
    println!("part2: {}", w1.max(w2));
}

const DICE: [usize; 27] = dice();

///Collection of the sums of dice permutations
const fn dice() -> [usize; 27] {
    let mut dice = [0; 27];
    let mut i = 0;
    let mut d1 = 1;
    while d1 <= 3 {
        let mut d2 = 1;
        while d2 <= 3 {
            let mut d3 = 1;
            while d3 <= 3 {
                dice[i] = d1 + d2 + d3;
                i += 1;
                d3 += 1;
            }
            d2 += 1;
        }
        d1 += 1;
    }
    dice
}

type GameState = Vec<Vec<Vec<Vec<Option<(usize, usize)>>>>>;

fn quantum_game(
    game_state: &mut GameState,
    p1: usize,
    p2: usize,
    pos1: usize,
    pos2: usize,
) -> (usize, usize) {
    if p1 >= 21 {
        (1, 0)
    } else if p2 >= 21 {
        (0, 1)
    } else if let Some(wins) = game_state[pos1][pos2][p1][p2] {
        wins
    } else {
        let mut wins = (0, 0);
        for d in DICE {
            let pos1 = (pos1 + d) % 10;
            let (w1, w2) = quantum_game(game_state, p2, p1 + pos1 + 1, pos2, pos1);
            wins.0 += w2;
            wins.1 += w1;
        }
        game_state[pos1][pos2][p1][p2] = Some(wins);
        wins
    }
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day21_part1(b: &mut Bencher) {
        let text = read_to_string("res/day21.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day21_part2(b: &mut Bencher) {
        let text = read_to_string("res/day21.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
