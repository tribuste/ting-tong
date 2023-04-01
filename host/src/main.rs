use methods::{TING_TONG_ID, TING_TONG_PATH};
use risc0_zkvm::{Prover, Receipt};
// use risc0_zkvm::serde::{from_slice, to_vec};

struct HonestServer{
    secret_guess: u64,
    secret_choice: u64,
}

impl HonestServer {
    pub fn get_secret(&self) -> Vec<u32> {
        // TODO: runs the prover and return the hash of secret values
        unimplemented!()
    }

    pub fn eval_round(&self, ) -> Receipt {
        // TODO: call the prover, i.e. run the guest code
        unimplemented!()
    }
}

struct Player {
    pub hash: Vec<u32>,
}

impl Player {
    pub fn check_receipt(&self, receipt: Receipt) -> Vec<u32> {
        // TODO: call the verify method and compare the stored hash and the hash
        // in the receipt
        unimplemented!()
    }
}

fn main() {
    // Make the prover.
    let method_code = std::fs::read(TING_TONG_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let mut prover = Prover::new(&method_code, TING_TONG_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    // TODO: Implement communication with the guest here

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(TING_TONG_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
}
