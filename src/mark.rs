use std::fmt;

/// An `X` or `O`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mark {
    X,
    O
}

impl Mark {
    /// Exchanges one `Mark` for the other.
    ///
    /// # Examples
    ///
    /// ```
    /// use xsos::Mark;
    ///
    /// let x = Mark::X;
    /// let o = Mark::O;
    ///
    /// assert_eq!(x.swap(), o);
    /// assert_eq!(o.swap(), x);
    /// ```
    pub fn swap(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::O => write!(f, "o")
        }
    }
}
