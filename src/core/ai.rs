use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::core::game::Game;
use crate::core::grid::Position;
use crate::core::referee::Outcome;

pub fn random_move(game: &Game) -> Position {
    let mut rng = thread_rng();
    moves(game).choose(&mut rng).cloned().unwrap()
}

pub fn moves(game: &Game) -> Vec<Position> {
    let positions = game.grid().available_positions().collect::<Vec<_>>();

    match positions.len() {
        0 | 1 | 9 => positions,
        _ => find_best_moves(&mut game.clone())
    }
}

fn find_best_moves(game: &mut Game) -> Vec<Position> {
    if let None = game.outcome() {
        let mut value = i8::MIN;
        let mut positions = Vec::new();

        for pos in game.grid().available_positions() {
            let mut next_game = game.clone();

            next_game.play(pos);

            let next_value = negamax(&mut next_game, -1);

            if next_value > value {
                value = next_value;
                positions.clear();
                positions.push(pos);
            } else if next_value == value {
                positions.push(pos);
            }
        }

        positions
    } else {
        Vec::new()
    }
}

fn negamax(game: &mut Game, color: i8) -> i8 {
    match game.outcome() {
        None => {
            let mut value = i8::MIN;

            for pos in game.grid().available_positions() {
                let mut next_game = game.clone();

                next_game.play(pos);

                value = std::cmp::max(value, color * negamax(&mut next_game, -color));
            }

            color * value
        },
        Some(outcome) => -(color * score(outcome))
    }
}

fn score(outcome: Outcome) -> i8 {
    match outcome {
        Outcome::Win => 2,
        Outcome::Squash => 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mark::Mark;

    #[test]
    fn it_finds_the_blocking_move_to_avoid_losing() {
        let mut game = Game::new(Mark::X);

        game.play((0, 0));
        game.play((0, 2));
        game.play((1, 1));

        assert_eq!(moves(&game), vec![(2, 2)]);
    }

    #[test]
    fn it_has_no_good_moves_since_every_position_is_losing() {
        let mut game = Game::new(Mark::X);

        game.play((0, 0));
        game.play((0, 1));
        game.play((1, 1));

        assert_eq!(moves(&game), vec![(0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
    }

    #[test]
    fn it_finds_the_winning_moves() {
        let mut game = Game::new(Mark::X);

        game.play((0, 0));
        game.play((0, 2));
        game.play((1, 0));
        game.play((2, 1));

        assert_eq!(moves(&game), vec![(1, 1), (1, 2), (2, 0), (2, 2)]);
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
