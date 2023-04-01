use std::io::{stdin, stdout, Write};

use methods::{TING_TONG_ID, TING_TONG_ELF};
use risc0_zkvm::{Prover, Receipt};
use risc0_zkvm::serde::to_vec;

use ting_tong_core::Guess;

struct HonestServer {
    secret: Guess 
}

impl HonestServer {
    // pub fn new_guess() -> Self {

    // }

    pub fn get_secret(&self) -> Vec<u32> {
        let dummy_guess = Guess {
            secret_choice: 0,
            secret_guess: 0,
        };

        let receipt = self.eval_round(dummy_guess);
        let journal = receipt.journal;
        journal[..16].to_owned()
    }

    pub fn eval_round(&self, player_guess: Guess) -> Receipt {
        let mut prover = Prover::new(TING_TONG_ELF, TING_TONG_ID).expect("failed to construct prover");

        prover.add_input_u32_slice(to_vec(&self.secret).unwrap().as_slice());
        prover.add_input_u32_slice(to_vec(&player_guess).unwrap().as_slice());

        return prover.run().unwrap();
    }
}

struct Player {
    pub hash: Vec<u32>,
}

impl Player {
    pub fn check_receipt(&self, receipt: Receipt) -> Vec<u32> {
        receipt
            .verify(TING_TONG_ID)
            .expect("receipt verification failed");

        let journal = receipt.journal;
        let hash = &journal[..16];

        if hash != self.hash {
            panic!("The server cheated!!!");
        }

        let result = &journal[16..];
        return result.to_owned();
    }
}

fn read_stdin_guess() -> Guess {
    let mut line = String::new();
    let mut guess = Guess {
        secret_choice: 0,
        secret_guess: 0,
    };

    loop {
        print!("Thumbs up!:");
        let _=stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.pop(); // remove trailing newline

        match line.parse::<u64>() {
            Ok(res) => {
                if res < 3 {
                    guess.secret_choice = res;
                    break;
                } else {
                    println!("WTF!? You have only 2 thumbs!!\n");
                    line.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Must by a number!!\n");
                line.clear();
                continue;
            }
        }
    };
    line.clear();
    loop {
        print!("What is your guess? How many thumbs will be up!?:");
        let _=stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.pop(); // remove trailing newline

        match line.parse::<u64>() {
            Ok(res) => {
                if res < 5 {
                    guess.secret_guess = res;
                    break;
                } else {
                    println!("2 players have only 4 thumbs in total!!\n");
                    line.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Must by a number!!\n");
                line.clear();
                continue;
            }
        }
    };

    guess
}

fn main() {
    println!("Let's play TING TONG!!");

    let mut game_won = false;

    while game_won == false {
        let guess_word = read_stdin_guess();
    }

}
