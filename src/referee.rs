use crate::grid::Grid;
use crate::mark::Mark;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Win,
    Squash
}

pub fn evaluate(grid: &Grid, mark: Mark) -> Option<Outcome> {
    if is_win(grid, mark) {
        Some(Outcome::Win)
    } else if is_squash(grid) {
        Some(Outcome::Squash)
    } else {
        None
    }
}

fn is_win(grid: &Grid, mark: Mark) -> bool {
    let cells = grid.cells().collect::<Vec<_>>();
    let c = Some(mark);

    ARRANGEMENTS.iter().any(|&(i, j, k)| (cells[i], cells[j], cells[k]) == (&c, &c, &c))
}

fn is_squash(grid: &Grid) -> bool {
    grid.cells().all(Option::is_some)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_on_an_empty_grid_returns_none() {
        let grid = Grid::new();

        assert!(evaluate(&grid, Mark::X).is_none());
        assert!(evaluate(&grid, Mark::O).is_none());
    }

    #[test]
    fn evaluate_detects_a_win() {
        let mut grid = Grid::new();

        grid.mark((0, 0), Mark::X);
        grid.mark((1, 0), Mark::O);
        grid.mark((0, 1), Mark::X);
        grid.mark((1, 1), Mark::O);
        grid.mark((0, 2), Mark::X);

        assert_eq!(evaluate(&grid, Mark::X), Some(Outcome::Win));
    }

    #[test]
    fn evaluate_detects_a_squash() {
        let mut grid = Grid::new();

        grid.mark((0, 0), Mark::X);
        grid.mark((1, 1), Mark::O);
        grid.mark((0, 1), Mark::X);
        grid.mark((0, 2), Mark::O);
        grid.mark((2, 0), Mark::X);
        grid.mark((1, 0), Mark::O);
        grid.mark((1, 2), Mark::X);
        grid.mark((2, 2), Mark::O);
        grid.mark((2, 1), Mark::X);

        assert_eq!(evaluate(&grid, Mark::X), Some(Outcome::Squash));
    }
}
