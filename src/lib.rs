#[derive(Debug, PartialEq)]
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
}
