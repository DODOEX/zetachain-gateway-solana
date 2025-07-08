## command 
cargo build-sbf -- -Znext-lockfile-bump
RUSTUP_TOOLCHAIN=nightly-2025-04-16 anchor build
solana program close account
solana program extend CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK 100000
solana program close-account BaNUCTTApi3CKgd2y6CkoFZkckH8jLzD257BTUGakH3o --program-id CbcR39gxjR2BH69ARzf5KF3tWSuNa9qpMaFSPecWgpNK
## client
cargo run deposit-sol-and-call 7000 1000000 0x4B37ff61e17DdcD4cEA80AF768de9455FC373764
cargo run deposit-spl-and-call 7000 EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v 100000 0x4B37ff61e17DdcD4cEA80AF768de9455FC373764