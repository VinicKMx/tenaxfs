# TenaxFS

TenaxFS is a deterministic transactional filesystem for embedded NOR flash.

It is designed for systems that need persistent state to remain correct after
power loss, brownout, watchdog reset, software crash, interrupted flash
programming, interrupted erase, storage pressure, wear imbalance, firmware
migration, and firmware rollback.

TenaxFS prioritizes:

- transactional correctness;
- power-loss recovery;
- bounded maintenance interference;
- observable flash wear;
- forensic recovery;
- explicit RAM and flash contracts.

The native API is object-oriented first:

- key-value objects for configuration, calibration, and metadata;
- streams for telemetry, events, logs, and audit data;
- immutable blobs for crash dumps, certificates, and firmware-related metadata;
- monotonic counters for operational generations and application epochs.

A traditional file API may be added later, but it must reuse the same
transactional storage core.

## Current Status

This repository is at checkpoint 1: project foundation.

Implemented now:

- Rust workspace;
- `tenaxfs-core` as a `no_std` crate;
- host flash/simulator scaffolding;
- host CLI scaffolding;
- invariants and architecture documentation;
- ADR structure;
- CI for formatting, build, test, clippy, and an embedded target check.

Not implemented yet:

- persistent record encoding;
- append-only log;
- transaction commit records;
- recovery;
- garbage collection;
- storage epochs;
- host image inspection.

## Workspace

```text
crates/
  tenaxfs-core/    no_std core types and contracts
  tenaxfs-flash/   flash backends and host NOR simulator
  tenaxfs-sim/     host simulation harness
  tenaxfs-cli/     host CLI entry point
docs/
  adr/             architecture decision records
```

## Development

Install the Rust toolchain declared in `rust-toolchain.toml`, then run:

```bash
cargo fmt --check
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p tenaxfs-core --target thumbv7em-none-eabihf
```

Run the current simulator smoke command:

```bash
cargo run -p tenaxfs-cli -- simulate
```

## Design Documents

- [Architecture](docs/architecture.md)
- [Invariants](docs/invariants.md)
- [Storage Format](docs/storage-format.md)
- [Transactions](docs/transactions.md)
- [Recovery](docs/recovery.md)
- [Maintenance](docs/maintenance.md)
- [Wear Leveling](docs/wear-leveling.md)
- [Storage Epochs](docs/epochs.md)
- [Power-Loss Model](docs/power-loss-model.md)
- [Performance](docs/performance.md)
- [Porting](docs/porting.md)

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option.
