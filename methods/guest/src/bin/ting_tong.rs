#![no_main]
#![no_std] // std support is experimental, but you can remove this to try it

risc0_zkvm::guest::entry!(main);
// use rand::Rng;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use ting_tong_core::{Guess, SECRET_GUESS_COUNT};

// fn random_number(n: u32) -> u32 {
//     let mut rng = rand::thread_rng();
//     rng.gen_range(0..n)
// }

pub fn check_guess(num_players: u64, players_choices: &[u64], guessed_number: u64) -> bool {
    // Check if players' choices are valid (0, 1, or 2)
    for &choice in players_choices {
        if choice > 2 {
            panic!(
                "Invalid choice: {}. Each player can vote for 0, 1, or 2.",
                choice
            );
        }
    }

    // Check if the number of players matches the choices given
    if num_players != players_choices.len() as u32 {
        panic!("Number of players does not match the number of choices.");
    }

    // Calculate the correct count
    let secret_number: u32 = players_choices.iter().sum();

    // Check if the guessed number is correct
    guessed_number == secret_number
}

pub fn main() {
    let num_players = 2;
    let mut server_count = 2;
    let mut player_count = 2;

    let server_guess: Guess = env::read();
    let player_guess: Guess = env::read();

    assert!(
        server_guess.secret_guess < 5,
        "Server secret_guess must be lower than 5."
    );

    assert!(
        player_guess.secret_guess < 5,
        "Player secret_guess must be lower than 5."
    );

    let player_result = check_guess(
        num_players,
        [*server_guess.secret_guess, *player_guess.secret_guess],
        player_guess.secret_choice,
    );

    let server_result = check_guess(
        num_players,
        [*server_guess.secret_guess, *player_guess.secret_guess],
        server_guess.secret_choice,
    );

    if server_result {
        server_count -= 1;
    }
    if player_result {
        player_count -= 1;
    }

    let server_hash = *Impl::hash_bytes(&server_guess.as_bytes());
    let game_state = GameState {
        server_hash,
        server_count,
        player_count,
    };

    env::commit(&game_state);
}
