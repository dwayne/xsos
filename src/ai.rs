use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::grid::Position;
use crate::game::Game;
use crate::referee::Outcome;

pub fn random_move(game: &Game) -> Position {
    let mut rng = thread_rng();
    moves(game).choose(&mut rng).cloned().unwrap()
}

pub fn moves(game: &Game) -> Vec<Position> {
    maximize(&mut game.clone(), 0).positions
}

fn maximize(game: &mut Game, depth: u32) -> Value {
    if game.is_playing() {
        let mut value = None;

        for pos in game.available_positions() {
            let mut next_game = game.clone();

            next_game.play(pos);

            let mut next_value = minimize(&mut next_game, depth + 1);
            next_value.positions = vec![pos];

            value = match value {
                None => Some(next_value),
                Some(v) => Some(v.max(next_value))
            }
        }

        value.unwrap()
    } else {
        Value::new(min_score(game), depth)
    }
}

fn minimize(game: &mut Game, depth: u32) -> Value {
    if game.is_playing() {
        let mut value = None;

        for pos in game.available_positions() {
            let mut next_game = game.clone();

            next_game.play(pos);

            let mut next_value = maximize(&mut next_game, depth + 1);
            next_value.positions = vec![pos];

            value = match value {
                None => Some(next_value),
                Some(v) => Some(v.min(next_value))
            }
        }

        value.unwrap()
    } else {
        Value::new(max_score(game), depth)
    }
}

fn max_score(game: &Game) -> i32 {
    match game.outcome().unwrap() {
        Outcome::Win => 2,
        Outcome::Squash => 1
    }
}

fn min_score(game: &Game) -> i32 {
    -max_score(game)
}

struct Value {
    score: i32,
    depth: u32,
    positions: Vec<Position>
}

impl Value {
    pub fn new(score: i32, depth: u32) -> Self {
        Self { score, depth, positions: Vec::new() }
    }

    pub fn max(self, other: Self) -> Self {
        if self.score > other.score {
            self
        } else if other.score > self.score {
            other
        } else if self.depth < other.depth {
            self
        } else if other.depth < self.depth {
            other
        } else {
            Self {
                positions: vec![self.positions, other.positions].concat(),
                ..self
            }
        }
    }

    pub fn min(self, other: Self) -> Self {
        if self.score < other.score {
            self
        } else if other.score < self.score {
            other
        } else if self.depth < other.depth {
            self
        } else if other.depth < self.depth {
            other
        } else {
            Self {
                positions: vec![self.positions, other.positions].concat(),
                ..self
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mark::Mark;

    #[test]
    fn it_finds_the_blocking_position_to_avoid_losing() {
        let mut game = Game::new(Mark::X);

        game.play((0, 0));
        game.play((0, 2));
        game.play((1, 1));

        assert_eq!(moves(&game), vec![(2, 2)]);
    }

    #[test]
    fn it_gives_up_when_losing_is_inevitable() {
        let mut game = Game::new(Mark::X);

        game.play((0, 0));
        game.play((0, 1));
        game.play((1, 1));

        assert_eq!(moves(&game), vec![(0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]);
    }

    #[test]
    fn it_finds_the_winning_position() {
        let mut game = Game::new(Mark::X);

        game.play((0, 0));
        game.play((0, 2));
        game.play((1, 0));
        game.play((2, 1));

        assert_eq!(moves(&game), vec![(2, 0)]);
    }

    #[test]
    fn it_favors_winning_over_blocking() {
        let mut game = Game::new(Mark::X);

        game.play((2, 0));
        game.play((0, 2));
        game.play((0, 0));
        game.play((2, 2));

        assert_eq!(moves(&game), vec![(1, 0)]);
    }
}
