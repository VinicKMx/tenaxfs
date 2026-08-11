# Storage Format

This document records the intended persistent format constraints. The final
binary layout will be derived from invariants, parser tests, and fault-injection
results.

## Encoding

Persistent fields are encoded explicitly. Rust structs must never be serialized
directly as the on-flash format.

All integer fields are little-endian unless this document is revised by an ADR.

Every persistent structure must carry enough version information to distinguish:

- unsupported old format;
- unsupported future format;
- corrupted format.

## Segment

The main logical flash unit is a segment. A segment should normally align to one
physical erase block or a deliberate group of erase blocks.

Conceptual segment contents:

```text
Segment Header
Record
Record
Commit Record
Record
Free erased space
```

The segment header is expected to contain:

- magic;
- format version;
- generation;
- erase count;
- state;
- sequence;
- checksum.

## Record

A record should contain only what is required for reconstruction and validation:

- magic;
- format version;
- record type;
- flags;
- object identifier;
- object version;
- transaction identifier;
- storage epoch;
- payload length;
- header checksum;
- payload checksum;
- payload.

## Checksums

Checksums detect accidental corruption and incomplete writes. They are not
cryptographic authentication.

