#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scisor,
}

enum Direction {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Winner {
    Player,
    Draw,
    Opponent,
}

impl Move {
    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scisor => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scisor,
            Self::Paper => Self::Rock,
            Self::Scisor => Self::Paper,
        }
    }

    fn winner(&self, other: &Self) -> Winner {
        if other == &self.beats() {
            Winner::Player
        } else if other == &self.beats().beats() {
            Winner::Opponent
        } else {
            Winner::Draw
        }
    }
}

impl Winner {
    fn points(&self) -> u32 {
        match self {
            Self::Opponent => 0,
            Self::Draw => 3,
            Self::Player => 6,
        }
    }
}

struct Round {
    player_direction: Direction,
    opponent_move: Move,
}

fn read_elves(input: &str) -> Option<Vec<Round>> {
    let mut game = Vec::new();
    for round_line in input.lines() {
        let (opponent_move_str, player_move_str) = round_line.split_once(' ')?;
        let opponent_move = match opponent_move_str {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scisor,
            _ => unimplemented!(),
        };

        let player_direction = match player_move_str {
            "X" => Direction::X,
            "Y" => Direction::Y,
            "Z" => Direction::Z,
            _ => unimplemented!(),
        };

        game.push(Round {
            player_direction,
            opponent_move,
        });
    }
    Some(game)
}

pub fn part_one(input: &str) -> u32 {
    let play_round = |round: &Round| {
        let player_move = match round.player_direction {
            Direction::X => Move::Rock,
            Direction::Y => Move::Paper,
            Direction::Z => Move::Scisor,
        };

        player_move.points() + round.opponent_move.winner(&player_move).points()
    };

    return read_elves(input).unwrap().iter().map(play_round).sum();
}

pub fn part_two(input: &str) -> u32 {
    let play_round = |round: &Round| {
        let outcome = match round.player_direction {
            Direction::X => Winner::Opponent,
            Direction::Y => Winner::Draw,
            Direction::Z => Winner::Player,
        };

        let player_move = match outcome {
            Winner::Player => round.opponent_move.beats().beats(),
            Winner::Draw => round.opponent_move,
            Winner::Opponent => round.opponent_move.beats(),
        };

        player_move.points() + outcome.points()
    };

    return read_elves(input).unwrap().iter().map(play_round).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 12);
    }
}
