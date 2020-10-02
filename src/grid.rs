use crate::mark::Mark;

const SIZE: usize = 3;
const NCELLS: usize = SIZE * SIZE;

/// The location of a [`Cell`] within a [`Grid`].
///
/// The first coordinate is the 0-based `row` and the second coordinate is the 0-based `column`.
///
/// # Examples
///
/// ```
/// use xsos::Position;
///
/// let p: Position = (1, 1);
/// ```
///
/// [`Cell`]: ./type.Cell.html
/// [`Grid`]: ./struct.Grid.html
pub type Position = (usize, usize);

/// An area within a [`Grid`] that may be marked with a [`Mark`].
///
/// # Examples
///
/// ```
/// use xsos::{ Cell, Mark };
///
/// // A cell that is marked with an X.
/// let cell: Cell = Some(Mark::X);
///
/// // An unmarked cell.
/// let cell: Cell = None;
/// ```
///
/// [`Grid`]: ./struct.Grid.html
/// [`Mark`]: ./enum.Mark.html
pub type Cell = Option<Mark>;

/// A 3x3 Tic-tac-toe grid.
///
/// <pre>
///   0   1   2
/// 0   |   |
///  ---+---+---
/// 1   |   |
///  ---+---+---
/// 2   |   |
/// </pre>
#[derive(Clone)]
pub struct Grid {
    cells: [Cell; NCELLS],
    last: Option<Mark>
}

impl Grid {
    /// Creates a new empty `Grid`.
    pub fn new() -> Self {
        Self { cells: [None; NCELLS], last: None }
    }

    /// Returns `true` if the given `Position` is within the bounds of a 3x3 grid, i.e. `r ∊ {0, 1, 2}` and `c ∊ {0, 1, 2}`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xsos::Grid;
    ///
    /// assert!(Grid::in_bounds((0, 0)));
    /// assert!(Grid::in_bounds((2, 2)));
    ///
    /// assert!(!Grid::in_bounds((3, 3)));
    /// assert!(!Grid::in_bounds((0, 3)));
    /// ```
    pub fn in_bounds((r, c): Position) -> bool {
        r < SIZE && c < SIZE
    }

    /// Marks a [`Cell`] at the given `Position` on this `Grid` with a `Mark`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xsos::{ Grid, Mark };
    ///
    /// let mut grid = Grid::new();
    ///
    /// // Mark the cell at (1, 1) with an X
    /// grid.mark((1, 1), Mark::X);
    ///
    /// // Mark the cell at (2, 2) with an O
    /// grid.mark((2, 2), Mark::O);
    ///
    /// assert!(grid.is_marked_at((1, 1)));
    /// assert!(grid.is_marked_at((2, 2)));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `Grid::in_bounds(p)` is `false`.
    ///
    /// [`Cell`]: ./type.Cell.html
    pub fn mark(&mut self, p: Position, m: Mark) {
        self.cells[to_index(p)] = Some(m);
        self.last = Some(m);
    }

    /// Returns `true` if the [`Cell`] at the given `Position` is marked.
    ///
    /// # Panics
    ///
    /// Panics if `Grid::in_bounds(p)` is `false`.
    ///
    /// [`Cell`]: ./type.Cell.html
    pub fn is_marked_at(&self, p: Position) -> bool {
        !self.is_unmarked_at(p)
    }

    /// Returns `true` if the [`Cell`] at the given `Position` is not marked.
    ///
    /// # Panics
    ///
    /// Panics if `Grid::in_bounds(p)` is `false`.
    ///
    /// [`Cell`]: ./type.Cell.html
    pub fn is_unmarked_at(&self, p: Position) -> bool {
        self.cells[to_index(p)].is_none()
    }

    /// Returns the last `Mark`, if any, to be marked on a [`Cell`].
    ///
    /// [`Cell`]: ./type.Cell.html
    pub fn last_mark(&self) -> Option<Mark> {
        self.last
    }

    /// Returns an iterator over the positions of the unmarked cells in this `Grid`.
    ///
    /// The positions are returned in [row-major order].
    ///
    /// ```
    /// use xsos::{ Grid, Mark };
    ///
    /// let mut grid = Grid::new();
    ///
    /// grid.mark((0, 0), Mark::X);
    /// grid.mark((0, 2), Mark::O);
    /// grid.mark((1, 1), Mark::X);
    /// grid.mark((2, 2), Mark::O);
    ///
    /// let mut unmarked_positions = grid.unmarked_positions();
    ///
    /// assert_eq!(unmarked_positions.next(), Some((0, 1)));
    /// assert_eq!(unmarked_positions.next(), Some((1, 0)));
    /// assert_eq!(unmarked_positions.next(), Some((1, 2)));
    /// assert_eq!(unmarked_positions.next(), Some((2, 0)));
    /// assert_eq!(unmarked_positions.next(), Some((2, 1)));
    /// assert_eq!(unmarked_positions.next(), None);
    /// ```
    ///
    /// [row-major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn unmarked_positions(&self) -> UnmarkedPositions {
        UnmarkedPositions::new(&self.cells)
    }

    /// Returns an iterator over the cells in this `Grid`.
    ///
    /// The cells are returned in [row-major order].
    ///
    /// ```
    /// use xsos::{ Grid, Mark };
    ///
    /// let mut grid = Grid::new();
    ///
    /// grid.mark((0, 0), Mark::X);
    /// grid.mark((0, 2), Mark::O);
    /// grid.mark((1, 1), Mark::X);
    /// grid.mark((2, 2), Mark::O);
    ///
    /// let mut cells = grid.cells();
    ///
    /// assert_eq!(cells.next(), Some(&Some(Mark::X)));
    /// assert_eq!(cells.next(), Some(&None));
    /// assert_eq!(cells.next(), Some(&Some(Mark::O)));
    /// assert_eq!(cells.next(), Some(&None));
    /// assert_eq!(cells.next(), Some(&Some(Mark::X)));
    /// assert_eq!(cells.next(), Some(&None));
    /// assert_eq!(cells.next(), Some(&None));
    /// assert_eq!(cells.next(), Some(&None));
    /// assert_eq!(cells.next(), Some(&Some(Mark::O)));
    /// assert_eq!(cells.next(), None);
    /// ```
    ///
    /// [row-major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn cells(&self) -> Cells {
        Cells::new(&self.cells)
    }
}

/// An iterator over the positions of the unmarked cells of a [`Grid`].
///
/// This struct is created by the [`unmarked_positions`] method on [`Grid`].
/// See its documentation for more.
///
/// [`unmarked_positions`]: ./struct.Grid.html#method.unmarked_positions
/// [`Grid`]: ./struct.Grid.html
pub struct UnmarkedPositions<'a> {
    cells: &'a [Cell],
    index: usize
}

impl<'a> UnmarkedPositions<'a> {
    fn new(cells: &'a [Cell]) -> Self {
        Self { cells, index: 0 }
    }
}

impl Iterator for UnmarkedPositions<'_> {
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

/// An iterator over the cells of a [`Grid`].
///
/// This struct is created by the [`cells`] method on [`Grid`].
/// See its documentation for more.
///
/// [`cells`]: ./struct.Grid.html#method.cells
/// [`Grid`]: ./struct.Grid.html
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

        grid.mark((0, 0), Mark::X);
        grid.mark((1, 1), Mark::O);

        assert!(grid.is_marked_at((0, 0)));
        assert!(grid.is_marked_at((1, 1)));

        assert_eq!(grid.unmarked_positions().collect::<Vec<_>>(), vec![
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

        grid.mark((0, 0), Mark::X);

        let mut clone_of_grid = grid.clone();

        clone_of_grid.mark((1, 1), Mark::O);

        assert!(clone_of_grid.is_marked_at((0, 0)));
        assert!(clone_of_grid.is_marked_at((1, 1)));
        assert!(grid.is_unmarked_at((1, 1)));
    }
}
