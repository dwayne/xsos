#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mark {
    X,
    O
}

impl Mark {
    pub fn next(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X
        }
    }
}

pub const SIZE: usize = 3;
pub const NCELLS: usize = SIZE * SIZE;

pub type Position = (usize, usize);
pub type Cell = Option<Mark>;

pub struct Grid {
    cells: [Cell; NCELLS]
}

impl Grid {
    pub fn new() -> Self {
        Self { cells: [None; NCELLS] }
    }

    pub fn in_bounds((r, c): Position) -> bool {
        r < SIZE && c < SIZE
    }

    pub fn set(&mut self, pos: Position, mark: Mark) {
        self.cells[Self::index(pos)] = Some(mark);
    }

    pub fn is_available(&self, pos: Position) -> bool {
        self.cells[Self::index(pos)].is_none()
    }

    pub fn available_positions(&self) -> Vec<Position> {
        // FIXME: Return an iterator.

        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| if cell.is_none() { Some(Self::pos(i)) } else { None })
            .collect()
    }

    pub fn cells(&self) -> Vec<&Cell> {
        // FIXME: Return an iterator.

        self.cells.iter().collect()
    }

    fn index((r, c): Position) -> usize {
        r * SIZE + c
    }

    fn pos(index: usize) -> Position {
        (index / SIZE, index % SIZE)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win,
    Squash
}

fn evaluate(grid: &Grid, mark: Mark) -> Option<Outcome> {
    let cells = grid.cells();

    if is_win(&cells, mark) {
        Some(Outcome::Win)
    } else if is_squash(&cells) {
        Some(Outcome::Squash)
    } else {
        None
    }
}

fn is_win(cells: &[&Cell], mark: Mark) -> bool {
    let t = Some(mark);

    ARRANGEMENTS.iter().any(|&(i, j, k)| (cells[i], cells[j], cells[k]) == (&t, &t, &t))
}

const ARRANGEMENTS: [(usize, usize, usize); 8] = [
    (0, 1, 2),
    (3, 4, 5),
    (6, 7, 8),
    (0, 3, 6),
    (1, 4, 7),
    (2, 5, 8),
    (0, 4, 8),
    (2, 4, 6)
];

fn is_squash(cells: &[&Cell]) -> bool {
    cells.iter().all(|cell| cell.is_some())
}

pub struct Game {
    grid: Grid,
    turn: Mark,
    state: State
}

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

        match evaluate(&self.grid, self.turn) {
            None => {
                self.turn = self.turn.next();
                self.state = State::Play(pos);
            },
            Some(outcome) => {
                self.state = State::GameOver(pos, outcome);
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, State::Start | State::Play(_))
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
    fn swap_x_returns_o() {
        assert_eq!(Mark::X.next(), Mark::O);
    }

    #[test]
    fn swap_o_returns_x() {
        assert_eq!(Mark::O.next(), Mark::X);
    }

    #[test]
    fn grid() {
        let mut grid = Grid::new();

        grid.set((0, 0), Mark::X);
        grid.set((1, 1), Mark::O);

        assert!(grid.is_available((0, 1)));
        assert!(!grid.is_available((0, 0)));
        assert!(!grid.is_available((1, 1)));

        assert_eq!(grid.available_positions(), vec![
            (0, 1), (0, 2),
            (1, 0), (1, 2),
            (2, 0), (2, 1), (2, 2)
        ]);

        assert_eq!(grid.cells(), vec![
            &Some(Mark::X), &None, &None,
            &None, &Some(Mark::O), &None,
            &None, &None, &None
        ]);
    }

    #[test]
    fn evaluate_on_an_empty_grid_returns_none() {
        let grid = Grid::new();

        assert!(evaluate(&grid, Mark::X).is_none());
        assert!(evaluate(&grid, Mark::O).is_none());
    }

    #[test]
    fn evaluate_detects_a_win() {
        let mut grid = Grid::new();

        grid.set((0, 0), Mark::X);
        grid.set((1, 0), Mark::O);
        grid.set((0, 1), Mark::X);
        grid.set((1, 1), Mark::O);
        grid.set((0, 2), Mark::X);

        assert_eq!(evaluate(&grid, Mark::X), Some(Outcome::Win));
    }

    #[test]
    fn evaluate_detects_a_squash() {
        let mut grid = Grid::new();

        grid.set((0, 0), Mark::X);
        grid.set((1, 1), Mark::O);
        grid.set((0, 1), Mark::X);
        grid.set((0, 2), Mark::O);
        grid.set((2, 0), Mark::X);
        grid.set((1, 0), Mark::O);
        grid.set((1, 2), Mark::X);
        grid.set((2, 2), Mark::O);
        grid.set((2, 1), Mark::X);

        assert_eq!(evaluate(&grid, Mark::X), Some(Outcome::Squash));
    }

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
}
