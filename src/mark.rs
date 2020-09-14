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
