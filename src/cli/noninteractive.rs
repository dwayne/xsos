use std::io::Write;

use crate::{ ai, Game, Mark, Outcome };

pub fn run(first: Mark, rounds: u8) {
    let mut game = Game::start(first);

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
            game.restart();
            break;
        }
    }
}

fn handle_game_over(outcome: Outcome, winner: Mark) {
    match outcome {
        Outcome::Win => print!("{}", winner),
        Outcome::Draw => print!(".")
    }
    std::io::stdout().flush().unwrap();
}
