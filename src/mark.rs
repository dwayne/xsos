use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::O => write!(f, "o")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_x_returns_o() {
        assert_eq!(Mark::X.next(), Mark::O);
    }

    #[test]
    fn next_o_returns_x() {
        assert_eq!(Mark::O.next(), Mark::X);
    }
}
