# Invariants

These invariants define the correctness contract for TenaxFS. Implementations,
tests, host tools, and future formal models should refer to these identifiers.

## INV-001

An uncommitted transaction never changes visible state.

## INV-002

A valid commit references only records that are fully valid.

## INV-003

After recovery, visible state corresponds to a valid sequence of commits.

## INV-004

No object has two simultaneously authoritative versions in the same logical
view.

## INV-005

A segment is never erased while it contains the only required copy of
authoritative data.

## INV-006

A checkpoint is never the only source of truth.

## INV-007

Loss or corruption of all checkpoints still allows reconstruction from the log,
except for physical damage outside the modeled failure assumptions.

## INV-008

A `RETIRED` segment never returns to the allocator.

## INV-009

The logical erase count never decreases.

## INV-010

After any modeled power loss, the filesystem must either mount or explicitly
return an unrecoverable media condition. It must never silently mount an
inconsistent state.

## INV-011

A candidate epoch transaction never irreversibly changes the confirmed epoch
before commit.

## INV-012

Recovery never mixes objects from incompatible epochs in the same confirmed
view.

## INV-013

The allocator preserves enough space to complete its own maintenance operations
according to configured invariants.

## INV-014

A foreground operation never starts an operation whose known worst case exceeds
its reserved resources without reporting that condition first.

