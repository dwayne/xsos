use std::io::Write;
use crate::ai;
use crate::game::Game;
use crate::mark::Mark;
use crate::referee::Outcome;

pub fn run(first: Mark, rounds: u8) {
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
