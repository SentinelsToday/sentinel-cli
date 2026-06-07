# sentinel-cli

**Operator CLI for the Sentinel Labs fleet.** Talks to [`sentinel-cloud`](https://github.com/Sentinels-Today/sentinel-cloud), generates device keys offline, and locally re-verifies the hash-chained audit log returned by the API.

[![ci](https://github.com/Sentinels-Today/sentinel-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Sentinels-Today/sentinel-cli/actions/workflows/ci.yml)
![license](https://img.shields.io/badge/license-Apache--2.0-blue)
![rust](https://img.shields.io/badge/rust-1.75%2B-orange)

## Install

```sh
cargo install --git https://github.com/Sentinels-Today/sentinel-cli
```

## Commands

```sh
sentinel keygen                              # offline Ed25519 keypair + DID
sentinel --cloud https://api.../ device did:sentinel:...
sentinel --cloud https://api.../ trust did:sentinel:...
sentinel --cloud https://api.../ audit did:sentinel:... # hash chain re-verified locally
sentinel version
```

Use `--output json` to emit machine-readable JSON instead of the default flat text. Use `--no-verify` on `audit` to skip the local hash-chain check.

## Develop

```sh
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

CI runs the same on ubuntu/macos/windows.

## License

Apache-2.0 — see [LICENSE](./LICENSE).
