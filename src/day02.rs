use std::io::{BufReader, Lines, Read};

enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::ROCK,
            'B' | 'Y' => RPS::PAPER,
            'C' | 'Z' => RPS::SCISSORS,
            _ => panic!()
        }
    }
}
impl From<i32> for RPS {
    fn from(i: i32) -> Self {
        match i.rem_euclid(3) {
            1 => RPS::ROCK,
            2 => RPS::PAPER,
            0 => RPS::SCISSORS,
            _ => panic!()
        }
    }
}

impl RPS {
    fn value(&self) -> i32 {
        match *self {
            RPS::ROCK => 1,
            RPS::PAPER => 2,
            RPS::SCISSORS => 3
        }
    }

    fn loses(&self) -> RPS {
        (self.value() + 1).into()
    }
    fn beats(&self) -> RPS {
        (self.value() - 1).into()
    }

    fn draws(&self) -> RPS {
        self.value().into()
    }
}

pub fn day02<T: Read + Sized>(lines: Lines<BufReader<T>>) -> (i32, i32) {
    let mut score_firstway = 0;
    let mut score_secondway = 0;
    for line in lines.filter_map(|result| result.ok()) {
        let opps: RPS = line.chars().nth(0).unwrap().into();
        let our_char = line.chars().nth(2).unwrap();

        score_firstway += get_score(&our_char.into(), &opps);
        let ours = match our_char {
            'X' => opps.beats(),
            'Y' => opps.draws(),
            'Z' => opps.loses(),
            _ => panic!()
        };
        score_secondway += get_score(&ours, &opps)
    }

    (score_firstway, score_secondway)
}

fn get_score(ours: &RPS, opps: &RPS) -> i32 {
    let selected_score = ours.value();
    let result_score = match (ours.value() - opps.value()).rem_euclid(3) {
        2 => 0, // loss
        0 => 3, // draw
        1 => 6, // win
        _ => panic!()
    };
    result_score + selected_score
}