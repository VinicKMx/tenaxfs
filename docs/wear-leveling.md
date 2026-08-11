# Wear Leveling

Wear is observable state, not hidden behavior.

TenaxFS should expose objective flash health data:

- total erases;
- minimum erase count;
- maximum erase count;
- median or average erase count when available;
- wear spread;
- retired segments;
- free segments;
- reclaimable segments;
- maintenance debt;
- write amplification estimate.

## Dynamic Wear Leveling

The allocator and garbage collector should avoid unnecessary concentration of
erase cycles.

## Static Wear Leveling

Static wear leveling may move cold data when wear imbalance justifies the write
amplification cost. It should be policy-driven and optional.

## Retired Segments

NOR flash does not have NAND-style bad blocks, but program, erase, and
verification failures can still happen. A segment may become `SUSPECT` and later
`RETIRED`. A retired segment never returns to the allocator.

