# Transactions

Transactions are central to TenaxFS.

A committed transaction must produce exactly one recovered state after modeled
power loss:

- the old state; or
- the fully committed new state.

It must never produce a partial new state.

## Commit Records

Logical visibility depends on a valid persistent commit record. Object records
written before the commit are not visible unless the matching commit survives
and validates.

Example:

```text
TX 1042
  Object record A
  Object record B
  Object record C
  Commit TX 1042
```

If power fails before a valid commit exists, transaction `1042` is ignored by
recovery.

## Admission

A transaction should declare its expected size before it is accepted:

```rust
TransactionSpec {
    max_records: 4,
    max_payload_bytes: 2048,
}
```

Accepted transactions reserve enough structural resources to finish without a
predictable space deadlock. Physical media failure remains a separate error
class.

