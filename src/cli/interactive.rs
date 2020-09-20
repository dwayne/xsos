use std::io::Write;
use crate::ai;
use crate::cli::Player;
use crate::game::{ Error, Game };
use crate::grid::{ Cell, Grid, Position };
use crate::mark::Mark;
use crate::referee::Outcome;

pub fn run(first: Mark, x: Player, o: Player) {
    println!("{}", format_intro());

    let mut game = Game::new(first);
    let humans = Player::count_humans(&[x, o]);

    loop {
        match game.turn() {
            Mark::X => play_one_game(&mut game, humans, x, o),
            Mark::O => play_one_game(&mut game, humans, o, x)
        }

        if read_continue() {
            game.renew();
        } else {
            break;
        }
    }
}

fn play_one_game(game: &mut Game, humans: u32, first: Player, second: Player) {
    let mut current = first;
    let mut next = second;

    loop {
        play_one_turn(game, humans, current);

        match game.outcome() {
            None => std::mem::swap(&mut current, &mut next),
            Some(outcome) => {
                handle_game_over(outcome, current, humans, game);
                break;
            }
        }
    }
}

fn play_one_turn(game: &mut Game, humans: u32, current: Player) {
    match current {
        Player::Human => {
            println!("{}", format_turn(humans, game.turn()));
            println!("{}", format_grid(game.grid()));

            loop {
                let pos = read_position(game.grid(), true);

                if let Some(error) = game.play(pos) {
                    match error {
                        Error::OutOfBounds => println!("Try again, that position is out of bounds"),
                        Error::Unavailable => println!("Try again, that position is already taken")
                    }
                } else {
                    break;
                }
            }
        },
        Player::Computer => {
            let pos = ai::random_move(game);

            game.play(pos);

            println!("The computer played at {}", format_position(pos));
        }
    }
}

fn handle_game_over(outcome: Outcome, player: Player, humans: u32, game: &Game) {
    match (outcome, player, humans) {
        (Outcome::Win, Player::Human, 2) => println!("Congratulations! {} won.", game.turn()),
        (Outcome::Win, Player::Human, 1) => println!("Congratulations! You won."),
        (Outcome::Win, Player::Computer, 1) => println!("The computer won. Better luck next time."),
        (Outcome::Squash, _, _) => println!("Game squashed."),
        _ => unreachable!()
    }

    println!("{}", format_grid(game.grid()));
}

fn first_available_position(grid: &Grid) -> Position {
    grid.available_positions()[0]
}

// INPUT

fn read_continue() -> bool {
    let input = read_input("Do you want to continue playing? (Y/n) ");
    let input = parse_yes_no(&input);

    match &input[..] {
        "" | "y" | "yes" => true,
        "n" | "no" => false,
        _ => read_continue()
    }
}

fn parse_yes_no(s: &str) -> String {
    s.to_ascii_lowercase()
}

fn read_position(grid: &Grid, show_hint: bool) -> Position {
    let input = read_input("> ");

    match parse_position(&input) {
        Some(pos) => pos,
        None => {
            if show_hint {
                let (r, c) = first_available_position(grid);

                println!("Try again, but this time enter a position in the format \"r c\",");
                println!("where 1 <= r <= 3 and 1 <= c <= 3, for e.g. \"{} {}\"", r + 1, c + 1);

                read_position(grid, false)
            } else {
                read_position(grid, show_hint)
            }
        }
    }
}

fn parse_position(s: &str) -> Option<Position> {
    let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

    match &parts[..] {
        &[a, b] => match (a.parse::<usize>(), b.parse::<usize>()) {
            (Ok(r), Ok(c)) if r > 0 && c > 0 => Some((r - 1, c - 1)),
            _ => None
        },
        _ => None
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();

    read_line(&mut buffer);

    buffer.trim().to_owned()
}

fn read_line(buffer: &mut String) {
    std::io::stdin().read_line(buffer).unwrap();
}

// OUTPUT

fn format_intro() -> String {
    format!("{}\n{}\n{}\n\n",
        "Welcome to Tic-tac-toe",
        "Play as many games as you want",
        "Press Ctrl-C to exit at any time"
    )
}

fn format_turn(humans: u32, mark: Mark) -> String {
    if humans == 2 {
        format!("{}'s turn", mark)
    } else {
        format!("Your turn ({})", mark)
    }
}

fn format_grid(grid: &Grid) -> String {
    let cells = grid.cells();
    let sep = "---+---+---";

    format!("{}\n{}\n{}\n{}\n{}",
        format_row(cells[0], cells[1], cells[2]),
        sep,
        format_row(cells[3], cells[4], cells[5]),
        sep,
        format_row(cells[6], cells[7], cells[8])
    )
}

fn format_row(a: &Cell, b: &Cell, c: &Cell) -> String {
    format!(" {} | {} | {}", format_cell(a), format_cell(b), format_cell(c))
}

fn format_cell(cell: &Cell) -> String {
    match cell {
        Some(mark) => mark.to_string(),
        None => String::from(" ")
    }
}

fn format_position((r, c): Position) -> String {
    format!("({}, {})", r+1, c+1)
}
