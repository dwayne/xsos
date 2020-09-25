use crate::core::mark::Mark;

const SIZE: usize = 3;
const NCELLS: usize = SIZE * SIZE;

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
        self.cells[to_index(pos)] = Some(mark);
    }

    pub fn is_available(&self, pos: Position) -> bool {
        self.cells[to_index(pos)].is_none()
    }

    pub fn available_positions(&self) -> AvailablePositions {
        AvailablePositions::new(&self.cells)
    }

    pub fn cells(&self) -> Cells {
        Cells::new(&self.cells)
    }
}

pub struct AvailablePositions<'a> {
    cells: &'a [Cell],
    index: usize
}

impl<'a> AvailablePositions<'a> {
    fn new(cells: &'a [Cell]) -> Self {
        Self { cells, index: 0 }
    }
}

impl Iterator for AvailablePositions<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < NCELLS && self.cells[self.index].is_some() {
            self.index += 1;
        }

        if self.index == NCELLS {
            None
        } else {
            self.index += 1;
            Some(to_pos(self.index - 1))
        }
    }
}

pub struct Cells<'a> {
    cells: &'a [Cell],
    index: usize
}

impl<'a> Cells<'a> {
    fn new(cells: &'a [Cell]) -> Self {
        Self { cells, index: 0 }
    }
}

impl<'a> Iterator for Cells<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < NCELLS {
            self.index += 1;
            Some(&self.cells[self.index - 1])
        } else {
            None
        }
    }
}

fn to_index((r, c): Position) -> usize {
    r * SIZE + c
}

fn to_pos(index: usize) -> Position {
    (index / SIZE, index % SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage() {
        let mut grid = Grid::new();

        grid.set((0, 0), Mark::X);
        grid.set((1, 1), Mark::O);

        assert!(!grid.is_available((0, 0)));
        assert!(!grid.is_available((1, 1)));

        assert_eq!(grid.available_positions().collect::<Vec<_>>(), vec![
            (0, 1), (0, 2),
            (1, 0), (1, 2),
            (2, 0), (2, 1), (2, 2)
        ]);

        assert_eq!(grid.cells().collect::<Vec<_>>(), vec![
            &Some(Mark::X), &None, &None,
            &None, &Some(Mark::O), &None,
            &None, &None, &None
        ]);
    }

    #[test]
    fn clone() {
        let mut grid = Grid::new();

        grid.set((0, 0), Mark::X);

        let mut clone_of_grid = grid.clone();

        clone_of_grid.set((1, 1), Mark::O);

        assert!(!clone_of_grid.is_available((0, 0)));
        assert!(!clone_of_grid.is_available((1, 1)));
        assert!(grid.is_available((1, 1)));
    }
}
