solana-test-validator --reset --ledger /tmp/test-ledger
cargo test -- --nocapture
solana program deploy target/deploy/called.so
solana program deploy target/deploy/caller.so

cargo fetch
cargo check -p tests
cargo test -p tests cross_programm_invocation_program_test -- --nocapture
cargo test -p tests cross_programm_invocation_validator_test -- --nocapture

cargo build-sbf --manifest-path=code/caller/Cargo.toml #optional# --features disable-entrypoint
cargo build-sbf --manifest-path=code/called/Cargo.toml #optional# --features disable-entrypoint
