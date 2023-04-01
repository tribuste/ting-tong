use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Guess {
    pub secret_guess: u64,
    pub secret_choice: u64,
}
