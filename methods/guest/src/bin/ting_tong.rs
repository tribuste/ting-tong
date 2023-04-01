#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
}
