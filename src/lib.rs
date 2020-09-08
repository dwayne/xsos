#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mark {
    X,
    O
}

impl Mark {
    pub fn swap(&self) -> Self {
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
enum Outcome {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_x_returns_o() {
        assert_eq!(Mark::X.swap(), Mark::O);
    }

    #[test]
    fn swap_o_returns_x() {
        assert_eq!(Mark::O.swap(), Mark::X);
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
}
