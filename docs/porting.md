# Porting

The TenaxFS core is `no_std` and should avoid mandatory heap allocation.

Ports must provide a flash implementation with:

- geometry;
- read;
- program;
- erase.

The initial physical model is NOR:

- erased bits read as `1`;
- programming changes bits only from `1` to `0`;
- erase restores an erase block to `1`;
- program and erase operations obey geometry alignment.

Backends should distinguish logical storage pressure from physical media
failure.

