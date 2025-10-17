# Solana CPI Example — Tests and Build Notes

This repository contains two example Solana programs (**caller** and **called**) and two corresponding tests:

* a **ProgramTest-based** test that runs in-process (`cross_program_invocation_program_test`), and
* a **validator-based** test that uses a running local validator and deployed programs (`cross_program_invocation_validator_test`).

> **Important:** Tests must now be run **separately!**
> There’s no point in running all tests together (e.g., `cargo test -- --nocapture`), because the ProgramTest-based test relies on `.so` binaries **without an entrypoint**, while the validator-based test requires `.so` files **with an entrypoint**.
> Therefore, each test must be built and run **separately**, with its own dedicated build.

---

## Validator-Based Test

### 1. **Build the Contracts**

First, build both contracts:

```bash
cargo build-sbf --manifest-path=code/caller/Cargo.toml
cargo build-sbf --manifest-path=code/called/Cargo.toml
```

> **Important:** The `--features disable-entrypoint` flag **must not** be used for validator-based tests.

---

### 2. **Start the Local Validator**

Launch a local Solana test validator:

```bash
solana-test-validator
```

> See `snippets.md` for an advanced validator launch command that helps avoid WSL-related issues.

---

### 3. **Deploy the Contracts**

Deploy both contracts to the local validator:

```bash
solana program deploy target/deploy/called.so
solana program deploy target/deploy/caller.so
```

> **Important:** After deployment, update the `pubkey` variables for the smart contracts in the tests with the actual addresses returned by the deploy commands.

---

### 4. **Run the Test**

Run the validator-based test:

```bash
cargo test -p tests cross_program_invocation_validator_test -- --nocapture
```

---

## ProgramTest-Based Test

### 1. **Build the Contracts**

First, build both contracts:

```bash
cargo build-sbf --manifest-path=code/caller/Cargo.toml --features disable-entrypoint
cargo build-sbf --manifest-path=code/called/Cargo.toml --features disable-entrypoint
```

> **Important:** The `--features disable-entrypoint` flag is **required** for ProgramTest-based tests.

---

### 2. **Run the Test**

Run the ProgramTest-based test:

```bash
cargo test -p tests cross_program_invocation_program_test -- --nocapture
```

---

> All useful command shortcuts are stored in `snippets.md`.

