# Recovery

Mount reconstructs visible state from persistent log records.

Conceptual flow:

```text
read segment headers
identify valid generations
load latest valid checkpoint if available
scan tail
validate records
apply valid commits
rebuild index
```

## Incomplete Records

Recovery must safely recognize and ignore:

- partial header;
- partial payload;
- partial commit;
- invalid checksum;
- invalid length.

Ignoring an incomplete record must not make uncommitted state visible.

## Checkpoints

Checkpoints are an optimization. They are not authoritative.

If a checkpoint is missing, stale, partial, or corrupted, recovery falls back to
an older checkpoint or a longer log scan.

Recovery decisions should be observable by host tools.

