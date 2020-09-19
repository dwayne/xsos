use std::io::Write;
use structopt::StructOpt;
use crate::ai;
use crate::game::Game;
use crate::mark::Mark;
use crate::referee::Outcome;

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Player {
    Human,
    Computer
}

fn parse_player(src: &str) -> Result<Player, &'static str> {
    match src {
        "human" => Ok(Player::Human),
        "computer" => Ok(Player::Computer),
        _ => Err("expected human|computer")
    }
}

fn parse_mark(src: &str) -> Result<Mark, &'static str> {
    match src {
        "x" => Ok(Mark::X),
        "o" => Ok(Mark::O),
        _ => Err("expected x|o")
    }
}

pub fn run(Config { x, o, first, rounds }: Config) {
    match (x, o) {
        (Player::Computer, Player::Computer) => noninteractive_run(first, rounds),
        _ => interactive_run(first, x, o)
    }
}

fn noninteractive_run(first: Mark, rounds: u8) {
    let mut game = Game::new(first);

    for _ in 0..rounds {
        play_one_round(&mut game);
    }

    if rounds > 0 {
        println!("");
    }
}

fn play_one_round(game: &mut Game) {
    loop {
        game.play(ai::random_move(game));

        if let Some(outcome) = game.outcome() {
            handle_game_over(outcome, game.turn());
            game.renew();
            break;
        }
    }
}

fn handle_game_over(outcome: Outcome, winner: Mark) {
    match outcome {
        Outcome::Win => print!("{:?}", winner),
        Outcome::Squash => print!(".")
    }
    std::io::stdout().flush().unwrap();
}

fn interactive_run(_first: Mark, _x: Player, _o: Player) {
    todo!()
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
        )
    }

    #[test]
    fn o_plays_first() {
        assert_eq!(
            Config::from_iter(&["", "--first", "o"]),
            Config {
                x: Player::Human,
                o: Player::Computer,
                first: Mark::O,
                rounds: 25
            }
        )
    }

    #[test]
    fn x_as_computer_o_as_human() {
        assert_eq!(
            Config::from_iter(&["", "-x", "computer", "-o", "human"]),
            Config {
                x: Player::Computer,
                o: Player::Human,
                first: Mark::X,
                rounds: 25
            }
        )
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
        )
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
        )
    }
}
