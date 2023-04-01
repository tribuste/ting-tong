use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

pub const SECRET_GUESS_COUNT: usize = 4;
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Guess {
    pub secret_guess: u64,
    pub secret_choice: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub server_hash: Digest,
    pub server_count: u32,
    pub player_count: u32,
}
