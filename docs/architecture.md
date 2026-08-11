# Architecture

TenaxFS is a `no_std` transactional filesystem and persistent storage engine
for embedded NOR flash.

The central engineering question is:

> Can embedded persistent storage provide transactional power-loss safety and
> bounded maintenance interference at the same time?

TenaxFS is not a POSIX-first filesystem. Its native surface is an embedded
object API backed by a transactional log-structured storage core.

## Product Pillars

TenaxFS is defined by the combination of:

- transactions;
- power-loss safety;
- budgeted maintenance;
- storage epochs;
- wear observability;
- forensic introspection.

## High-Level Layers

```text
Application APIs
  KV, Stream, Blob, Counter, future File API

Transaction Layer

Storage Epoch and Snapshot Layer

Object Index

Log-Structured Record Layer

Maintenance Engine
  GC, wear, checkpoints, rebalancing

Flash Abstraction
  NOR, simulator, fault injector
```

## Native Objects

The native API is specialized for embedded persistent state:

- key-value objects for configuration, calibration, state, and metadata;
- streams for telemetry, events, logs, history, alarms, and audit data;
- immutable blobs for crash dumps, certificates, assets, and package metadata;
- monotonic counters for operational generations and event identifiers.

The future file API must reuse these mechanisms rather than create another
storage engine.

## Flash Model

The initial target is NOR flash:

- SPI NOR;
- QSPI NOR;
- internal NOR.

NAND support is a future backend concern. ECC, OOB data, bad-block markers,
read disturb, and scrubbing must not contaminate the initial core model.

## Memory Model

The core must not require hidden heap allocation. RAM use should be explicit and
configurable by the embedding application.

## Authority Model

The authoritative state is the committed log. Checkpoints accelerate mount but
are never the only source of truth.

