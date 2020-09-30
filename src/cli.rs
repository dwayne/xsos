use structopt::StructOpt;

use crate::Mark;

mod interactive;
mod noninteractive;

#[derive(StructOpt, Debug, PartialEq, Clone, Copy)]
pub struct Config {
    #[structopt
        ( short
        , default_value = "human"
        , parse(try_from_str = parse_player)
        )
    ]
    x: Player,

    #[structopt
        ( short
        , default_value = "computer"
        , parse(try_from_str = parse_player)
        )
    ]
    o: Player,

    #[structopt
        ( short
        , long
        , default_value = "x"
        , parse(try_from_str = parse_mark)
        )
    ]
    first: Mark,

    #[structopt
        ( short
        , long
        , default_value = "25"
        )
    ]
    rounds: u8
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    Human,
    Computer
}

impl Player {
    pub fn count_humans(players: &[Self]) -> u32 {
        players.iter().fold(0, |sum, &p| if let Player::Human = p { sum + 1 } else { sum })
    }
}

fn parse_player(src: &str) -> Result<Player, &'static str> {
    match src.to_ascii_lowercase().as_ref() {
        "h" | "human" => Ok(Player::Human),
        "c" | "computer" => Ok(Player::Computer),
        _ => Err("expected human|computer")
    }
}

fn parse_mark(src: &str) -> Result<Mark, &'static str> {
    match src.to_ascii_lowercase().as_ref() {
        "x" => Ok(Mark::X),
        "o" => Ok(Mark::O),
        _ => Err("expected x|o")
    }
}

pub fn run() {
    let Config { x, o, first, rounds } = Config::from_args();

    if let (Player::Computer, Player::Computer) = (x, o) {
        noninteractive::run(first, rounds);
    } else {
        interactive::run(first, x, o);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        assert_eq!(
            Config::from_iter(&[""]),
            Config {
                x: Player::Human,
                o: Player::Computer,
                first: Mark::X,
                rounds: 25
            }
        );
    }

    #[test]
    fn let_o_play_first() {
        assert_eq!(
            Config::from_iter(&["", "--first", "o"]),
            Config {
                x: Player::Human,
                o: Player::Computer,
                first: Mark::O,
                rounds: 25
            }
        );
    }

    #[test]
    fn let_computer_play_with_x_and_human_play_with_o() {
        assert_eq!(
            Config::from_iter(&["", "-x", "computer", "-o", "human"]),
            Config {
                x: Player::Computer,
                o: Player::Human,
                first: Mark::X,
                rounds: 25
            }
        );

        // Shorthand
        assert_eq!(
            Config::from_iter(&["", "-x", "c", "-o", "h"]),
            Config {
                x: Player::Computer,
                o: Player::Human,
                first: Mark::X,
                rounds: 25
            }
        );

        // Case insensitive
        assert_eq!(
            Config::from_iter(&["", "-x", "cOmPuTeR", "-o", "H"]),
            Config {
                x: Player::Computer,
                o: Player::Human,
                first: Mark::X,
                rounds: 25
            }
        );
    }

    #[test]
    fn computer_vs_computer() {
        assert_eq!(
            Config::from_iter(&["", "-x", "computer"]),
            Config {
                x: Player::Computer,
                o: Player::Computer,
                first: Mark::X,
                rounds: 25
            }
        );
    }

    #[test]
    fn computer_vs_computer_for_50_rounds() {
        assert_eq!(
            Config::from_iter(&["", "-x", "computer", "--rounds", "50"]),
            Config {
                x: Player::Computer,
                o: Player::Computer,
                first: Mark::X,
                rounds: 50
            }
        );
    }
}
