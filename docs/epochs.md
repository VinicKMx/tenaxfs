# Storage Epochs

A storage epoch is a coherent persistent view of state.

Example:

```text
Epoch 41 confirmed
Epoch 42 candidate
```

Candidate epochs support firmware trials and schema migrations without
irreversibly changing confirmed state before commit.

## Visibility

Reads in a candidate epoch see:

- the candidate object version when present;
- otherwise the inherited confirmed version.

This allows efficient copy-on-write migrations without duplicating the whole
flash image.

## Commit

Epoch commit is transactional. Power loss during epoch commit must recover
either the old authoritative epoch or the new authoritative epoch, never a mixed
view.

## Rollback

Rollback discards the candidate view logically. Candidate data can become
garbage and be collected later.

## Firmware Integration

TenaxFS must not depend on a bootloader. Integration with Rampart or another
firmware lifecycle manager should be implemented as an adapter.

