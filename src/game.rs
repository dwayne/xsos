use crate::mark::Mark;
use crate::grid::{ Grid, Position };
use crate::referee::{ self, Outcome };

#[derive(Clone)]
pub struct Game {
    grid: Grid,
    turn: Mark,
    state: State
}

#[derive(Clone, Copy)]
enum State {
    Start,
    Play(Position),
    GameOver(Position, Outcome)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
    Unavailable
}

impl Game {
    pub fn new(first: Mark) -> Self {
        Self {
            grid: Grid::new(),
            turn: first,
            state: State::Start
        }
    }

    pub fn play(&mut self, pos: Position) -> Option<Error> {
        match self.state {
            State::Start | State::Play(_) => {
                if Grid::in_bounds(pos) {
                    if self.grid.is_available(pos) {
                        self.unchecked_play(pos);
                        None
                    } else {
                        Some(Error::Unavailable)
                    }
                } else {
                    Some(Error::OutOfBounds)
                }
            },
            State::GameOver(..) => None
        }
    }

    fn unchecked_play(&mut self, pos: Position) {
        self.grid.set(pos, self.turn);

        match referee::evaluate(&self.grid, self.turn) {
            None => {
                self.turn = self.turn.next();
                self.state = State::Play(pos);
            },
            Some(outcome) => {
                self.state = State::GameOver(pos, outcome);
            }
        }
    }

    pub fn available_positions(&self) -> Vec<Position> {
        self.grid.available_positions()
    }

    pub fn is_playing(&self) -> bool {
        !self.is_game_over()
    }

    pub fn is_game_over(&self) -> bool {
        matches!(self.state, State::GameOver(..))
    }

    pub fn outcome(&self) -> Option<Outcome> {
        if let State::GameOver(_, outcome) = self.state {
            Some(outcome)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_after_3_plays() {
        let mut game = Game::new(Mark::X);

        assert!(game.is_playing());

        game.play((1, 1));
        game.play((0, 2));
        game.play((2, 0));

        assert!(game.is_playing());
        assert_eq!(game.turn, Mark::O);
        assert_eq!(game.grid.cells(), vec![
            &None, &None, &Some(Mark::O),
            &None, &Some(Mark::X), &None,
            &Some(Mark::X), &None, &None
        ]);
    }

    #[test]
    fn game_when_x_wins() {
        let mut game = Game::new(Mark::X);

        game.play((1, 1));
        game.play((0, 2));
        game.play((2, 0));
        game.play((1, 2));
        game.play((2, 2));
        game.play((2, 1));
        game.play((0, 0));

        assert!(game.is_game_over());
        assert_eq!(game.turn, Mark::X);
        assert_eq!(game.grid.cells(), vec![
            &Some(Mark::X), &None, &Some(Mark::O),
            &None, &Some(Mark::X), &Some(Mark::O),
            &Some(Mark::X), &Some(Mark::O), &Some(Mark::X)
        ]);
        assert_eq!(game.outcome().unwrap(), Outcome::Win);
    }

    #[test]
    fn game_when_o_squashes() {
        let mut game = Game::new(Mark::O);

        game.play((1, 1));
        game.play((0, 0));
        game.play((2, 2));
        game.play((0, 2));
        game.play((0, 1));
        game.play((2, 1));
        game.play((1, 2));
        game.play((1, 0));
        game.play((2, 0));

        assert!(game.is_game_over());
        assert_eq!(game.turn, Mark::O);
        assert_eq!(game.grid.cells(), vec![
            &Some(Mark::X), &Some(Mark::O), &Some(Mark::X),
            &Some(Mark::X), &Some(Mark::O), &Some(Mark::O),
            &Some(Mark::O), &Some(Mark::X), &Some(Mark::O)
        ]);
        assert_eq!(game.outcome().unwrap(), Outcome::Squash);
    }

    #[test]
    fn game_when_position_is_out_of_bounds() {
        let mut game = Game::new(Mark::X);

        assert_eq!(game.play((0, 4)), Some(Error::OutOfBounds));
    }

    #[test]
    fn game_when_position_is_unavailable() {
        let mut game = Game::new(Mark::X);

        game.play((1, 1));

        assert_eq!(game.play((1, 1)), Some(Error::Unavailable));
    }

    #[test]
    fn a_game_can_be_cloned() {
        let mut game = Game::new(Mark::X);

        game.play((1, 1));
        game.play((0, 2));
        game.play((2, 0));
        game.play((1, 2));
        game.play((2, 2));
        game.play((2, 1));

        let mut clone_of_game = game.clone();

        clone_of_game.play((0, 0));

        assert!(game.is_playing());
        assert!(clone_of_game.is_game_over());
    }
}
