# Maintenance

TenaxFS maintenance is incremental by design.

Garbage collection, copying, erasing, checkpointing, and wear rebalancing must
be visible to the maintenance scheduler. Foreground operations should not hide
unbounded maintenance work.

## Budget Model

Budgets should express observable operations rather than unverifiable timing
claims:

- maximum copied bytes;
- maximum program operations;
- erase permission;
- optional backend time estimates.

Example:

```text
max copy       256 B
max programs   3
erase allowed  no
```

## Reserved Segments

The allocator must preserve erased segment reserves so maintenance can make
progress:

- active append segment;
- GC destination segment;
- emergency reserve.

This prevents the filesystem from needing free space in order to create free
space.

## Storage Pressure

Applications must be able to observe storage pressure before the filesystem
reaches a critical condition:

- normal;
- elevated;
- high;
- critical;
- maintenance required.

