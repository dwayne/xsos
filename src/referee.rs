use crate::grid::Grid;

/// A `Win` or `Draw`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Win,
    Draw
}

/// Determines the `Outcome`, if any, of a given `Grid`.
pub fn evaluate(grid: &Grid) -> Option<Outcome> {
    if is_win(grid) {
        Some(Outcome::Win)
    } else if is_draw(grid) {
        Some(Outcome::Draw)
    } else {
        None
    }
}

fn is_win(grid: &Grid) -> bool {
    let cells = grid.cells().collect::<Vec<_>>();
    let c = grid.last_mark();

    c.is_some() && ARRANGEMENTS.iter().any(|&(i, j, k)| (cells[i], cells[j], cells[k]) == (&c, &c, &c))
}

fn is_draw(grid: &Grid) -> bool {
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
    use crate::mark::Mark;

    #[test]
    fn evaluate_on_an_empty_grid_returns_none() {
        let grid = Grid::new();

        assert!(evaluate(&grid).is_none());
    }

    #[test]
    fn evaluate_detects_a_win() {
        let mut grid = Grid::new();

        grid.mark((0, 0), Mark::X);
        grid.mark((1, 0), Mark::O);
        grid.mark((0, 1), Mark::X);
        grid.mark((1, 1), Mark::O);
        grid.mark((0, 2), Mark::X);

        assert_eq!(evaluate(&grid), Some(Outcome::Win));
    }

    #[test]
    fn evaluate_detects_a_draw() {
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

        assert_eq!(evaluate(&grid), Some(Outcome::Draw));
    }
}
