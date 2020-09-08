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

    pub fn cells(&self) -> Vec<(Position, &Cell)> {
        // FIXME: Return an iterator.

        self.cells
            .iter()
            .enumerate()
            .map(|(i, cell)| (Self::pos(i), cell))
            .collect()
    }

    fn index((r, c): Position) -> usize {
        r * SIZE + c
    }

    fn pos(index: usize) -> Position {
        (index / SIZE, index % SIZE)
    }
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
            ((0, 0), &Some(Mark::X)), ((0, 1), &None), ((0, 2), &None),
            ((1, 0), &None), ((1, 1), &Some(Mark::O)), ((1, 2), &None),
            ((2, 0), &None), ((2, 1), &None), ((2, 2), &None)
        ]);
    }
}
