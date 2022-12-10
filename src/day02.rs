enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            _ => panic!()
        }
    }
}

impl From<i32> for RPS {
    fn from(i: i32) -> Self {
        match i.rem_euclid(3) {
            1 => RPS::Rock,
            2 => RPS::Paper,
            0 => RPS::Scissors,
            _ => panic!()
        }
    }
}

impl RPS {
    fn value(&self) -> i32 {
        match *self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
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

pub fn day02(lines: Vec<String>) -> (i32, i32) {
    let mut score_firstway = 0;
    let mut score_secondway = 0;
    for line in lines {
        let opps: RPS = line.chars().next().unwrap().into();
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


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_02() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day02.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day02(lines), (14531, 11258));
    }
}