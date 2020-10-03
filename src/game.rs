use crate::grid::{ Grid, Position };
use crate::mark::Mark;
use crate::referee::{ self, Outcome };

/// The game logic for Tic-tac-toe.
///
/// It guarantees that the *rules of Tic-tac-toe* are enforced and
/// that a *valid grid* is maintained throughout game play.
///
/// # What are the rules of Tic-tac-toe?
///
/// Either X or O plays first and then play alternates until the
/// game is over, either by virtue of a win or draw.
///
/// # What's a valid grid?
///
/// It's any grid configuration that's possible to reach by strictly
/// following the rules of Tic-tac-toe.
///
/// # Examples
///
/// ```
/// use xsos::{ Game, Mark, Outcome, PlayError };
///
/// // Start a new game and let X play first
/// let mut game = Game::start(Mark::X);
///
/// // Mark X at (0, 2)
/// game.play((0, 2));
///
/// // Mark O at (1, 2)
/// game.play((1, 2));
///
/// // Notice that you don't pass the mark since that's managed internally
///
/// assert_eq!(game.turn(), Mark::X);
///
/// // Mark X at (1, 1)
/// game.play((1, 1));
///
/// assert_eq!(game.turn(), Mark::O);
///
/// // Try to mark O on a marked cell
/// assert_eq!(game.play((1, 1)), Some(PlayError::AlreadyMarked));
///
/// // Still O's turn
/// // Try to mark O at some position outside the grid
/// assert_eq!(game.play((3, 3)), Some(PlayError::OutOfBounds));
///
/// // Still O's turn, so mark O at (2, 2)
/// game.play((2, 2));
///
/// assert!(game.is_playing());
///
/// // Take the win by marking X at (2, 0)
/// game.play((2, 0));
///
/// //   0   1   2
/// // 0   |   | x
/// //  ---+---+---
/// // 1   | x | o
/// //  ---+---+---
/// // 2 x |   | o
///
/// // The game is over and X won
/// assert!(game.is_game_over());
/// assert_eq!(game.turn(), Mark::X);
/// assert_eq!(game.outcome(), Some(Outcome::Win));
///
/// // Want to play another? Just restart the game. The player that won
/// // gets to play first in the restarted game. However, if the game
/// // ended in a draw then the other player gets to play first next time
/// // around
/// game.restart();
///
/// assert!(game.is_playing());
/// assert_eq!(game.turn(), Mark::X);
/// assert_eq!(game.outcome(), None);
///
/// // Let's draw this one
/// game.play((1, 1));
/// game.play((0, 0));
/// game.play((0, 1));
/// game.play((2, 1));
/// game.play((2, 0));
/// game.play((0, 2));
/// game.play((1, 2));
/// game.play((1, 0));
/// game.play((2, 2));
///
/// assert!(game.is_game_over());
/// assert_eq!(game.outcome(), Some(Outcome::Draw));
///
/// game.restart();
///
/// // See how O gets to play first this time around
/// assert_eq!(game.turn(), Mark::O);
/// ```
#[derive(Clone)]
pub struct Game {
    grid: Grid,
    turn: Mark,
    state: State
}

#[derive(Clone, Copy)]
enum State {
    Play,
    GameOver(Outcome)
}

/// The possible errors that can occur when playing a [`Game`].
///
/// [`Game`]: ./struct.Game.html
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayError {
    /// Tried to mark a marked cell.
    AlreadyMarked,

    /// Tried to play at a position, `p`, such that `Grid::in_bounds(p) == false`.
    OutOfBounds
}

impl Game {
    /// Start a new game and let `first` play first.
    ///
    /// # Examples
    ///
    /// ```
    /// use xsos::{ Game, Mark };
    ///
    /// let game = Game::start(Mark::X);
    ///
    /// assert_eq!(game.turn(), Mark::X);
    /// ```
    pub fn start(first: Mark) -> Self {
        Self {
            grid: Grid::new(),
            turn: first,
            state: State::Play
        }
    }

    /// Restart a game.
    pub fn restart(&mut self) {
        self.grid = Grid::new();

        if let State::GameOver(Outcome::Draw) = self.state {
            self.turn = self.turn.swap();
        }

        self.state = State::Play;
    }

    /// Marks the [`Cell`] at the given [`Position`] on the [`Grid`] managed by this `Game`, say `game`,
    /// with the [`Mark`] given by `game.turn()`, unless it's game over.
    ///
    /// It returns:
    ///
    /// - `Some(PlayError::OutOfBounds)`, if this `Game` is not over and `p` is out of bounds.
    /// - `Some(PlayError::AlreadyMarked)`, if this `Game` is not over and the [`Cell`] at `p` is already marked.
    /// - `None`, in all other cases.
    ///
    /// # Examples
    ///
    /// ```
    /// use xsos::{ Game, Mark, PlayError };
    ///
    /// let mut game = Game::start(Mark::X);
    ///
    /// assert_eq!(game.play((3, 3)), Some(PlayError::OutOfBounds));
    /// assert_eq!(game.play((0, 0)), None);
    /// assert_eq!(game.play((0, 0)), Some(PlayError::AlreadyMarked));
    ///
    /// game.play((0, 1));
    /// game.play((1, 0));
    /// game.play((1, 1));
    /// game.play((2, 0));
    ///
    /// assert!(game.is_game_over());
    ///
    /// // You can't make any plays once the game is over
    ///
    /// assert_eq!(game.play((2, 1)), None);
    /// assert!(game.grid().is_unmarked_at((2, 1)));
    /// ```
    ///
    /// [`Cell`]: ./type.Cell.html
    /// [`Grid`]: ./struct.Grid.html
    /// [`Mark`]: ./enum.Mark.html
    /// [`Position`]: ./type.Position.html
    pub fn play(&mut self, p: Position) -> Option<PlayError> {
        if self.is_playing() {
            if Grid::in_bounds(p) {
                if self.grid.is_unmarked_at(p) {
                    unchecked_play(self, p);
                    None
                } else {
                    Some(PlayError::AlreadyMarked)
                }
            } else {
                Some(PlayError::OutOfBounds)
            }
        } else {
            None
        }
    }

    /// Returns `true` if this `Game` is in play.
    pub fn is_playing(&self) -> bool {
        matches!(self.state, State::Play)
    }

    /// Returns `true` if this `Game` is over.
    pub fn is_game_over(&self) -> bool {
        !self.is_playing()
    }

    /// Returns the [`Grid`] managed by this `Game`.
    ///
    /// [`Grid`]: ./struct.Grid.html
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Returns the answer to the question "Who's turn is it to play?".
    pub fn turn(&self) -> Mark {
        self.turn
    }

    /// Returns a reason for why this `Game` is over, if it is over. Otherwise it returns `None`.
    pub fn outcome(&self) -> Option<Outcome> {
        if let State::GameOver(outcome) = self.state {
            Some(outcome)
        } else {
            None
        }
    }
}

pub fn unchecked_play(game: &mut Game, p: Position) {
    game.grid.mark(p, game.turn);

    if let Some(outcome) = referee::evaluate(&game.grid) {
        game.state = State::GameOver(outcome);
    } else {
        game.turn = game.turn.swap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_3_plays() {
        let mut game = Game::start(Mark::X);

        assert!(game.is_playing());

        game.play((1, 1));
        game.play((0, 2));
        game.play((2, 0));

        assert!(game.is_playing());
        assert_eq!(game.turn, Mark::O);
        assert_eq!(game.grid().cells().collect::<Vec<_>>(), vec![
            &None, &None, &Some(Mark::O),
            &None, &Some(Mark::X), &None,
            &Some(Mark::X), &None, &None
        ]);
    }

    #[test]
    fn when_x_wins() {
        let mut game = Game::start(Mark::X);

        game.play((1, 1));
        game.play((0, 2));
        game.play((2, 0));
        game.play((1, 2));
        game.play((2, 2));
        game.play((2, 1));
        game.play((0, 0));

        assert!(game.is_game_over());
        assert_eq!(game.turn, Mark::X);
        assert_eq!(game.grid().cells().collect::<Vec<_>>(), vec![
            &Some(Mark::X), &None, &Some(Mark::O),
            &None, &Some(Mark::X), &Some(Mark::O),
            &Some(Mark::X), &Some(Mark::O), &Some(Mark::X)
        ]);
        assert_eq!(game.outcome(), Some(Outcome::Win));
    }

    #[test]
    fn when_o_squashes() {
        let mut game = Game::start(Mark::O);

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
        assert_eq!(game.grid().cells().collect::<Vec<_>>(), vec![
            &Some(Mark::X), &Some(Mark::O), &Some(Mark::X),
            &Some(Mark::X), &Some(Mark::O), &Some(Mark::O),
            &Some(Mark::O), &Some(Mark::X), &Some(Mark::O)
        ]);
        assert_eq!(game.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn when_position_is_out_of_bounds() {
        let mut game = Game::start(Mark::X);

        assert_eq!(game.play((0, 4)), Some(PlayError::OutOfBounds));
    }

    #[test]
    fn when_position_is_unavailable() {
        let mut game = Game::start(Mark::X);

        game.play((1, 1));

        assert_eq!(game.play((1, 1)), Some(PlayError::AlreadyMarked));
    }

    #[test]
    fn clone() {
        let mut game = Game::start(Mark::X);

        game.play((1, 1));
        game.play((0, 2));
        game.play((2, 0));
        game.play((1, 2));
        game.play((2, 2));
        game.play((2, 1));

        let mut clone_of_game = game.clone();

        clone_of_game.play((0, 0));

        assert!(clone_of_game.is_game_over());
        assert!(game.is_playing());
    }
}
