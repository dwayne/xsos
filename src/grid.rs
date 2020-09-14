use crate::mark::Mark;

pub const SIZE: usize = 3;
pub const NCELLS: usize = SIZE * SIZE;

pub type Position = (usize, usize);
pub type Cell = Option<Mark>;

#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn a_grid_can_be_cloned() {
        let mut grid = Grid::new();

        grid.set((0, 0), Mark::X);

        let mut clone_of_grid = grid.clone();

        clone_of_grid.set((1, 1), Mark::O);

        assert!(!clone_of_grid.is_available((0, 0)));
        assert!(!clone_of_grid.is_available((1, 1)));
        assert!(grid.is_available((1, 1)));

    }
}
