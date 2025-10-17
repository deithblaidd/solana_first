### Launch Command for Solana Test Validator (WSL Configuration Workaround)

```bash
solana-test-validator --reset --ledger /tmp/test-ledger
```

### Shortcuts for Program Deployment

```bash
solana program deploy target/deploy/called.so
solana program deploy target/deploy/caller.so
```

### Shortcuts for Updating Dependency Versions in Containers

```bash
cargo fetch
cargo check -p tests
```

### Shortcuts for Builds

*(Use `--features disable-entrypoint` for ProgramTest-based tests)*

```bash
cargo build-sbf --manifest-path=code/caller/Cargo.toml #optional# --features disable-entrypoint
cargo build-sbf --manifest-path=code/called/Cargo.toml #optional# --features disable-entrypoint
```

### Shortcuts for Tests

```bash
cargo test -p tests cross_programm_invocation_program_test -- --nocapture
cargo test -p tests cross_programm_invocation_validator_test -- --nocapture
```

### Run All Tests at Once *(no longer used)*

```bash
cargo test -- --nocapture
```