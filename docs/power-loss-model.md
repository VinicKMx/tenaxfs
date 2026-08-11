# Power-Loss Model

The simulator and test harness should model power loss at persistent operation
boundaries and inside operations where the backend can fail partially.

Fault points include:

- before program;
- after program;
- during program;
- before erase;
- after erase;
- during erase;
- between metadata writes;
- during commit;
- during checkpoint;
- during garbage collection;
- during epoch commit.

For small operations, exhaustive fault injection should test every persistent
action boundary:

```text
run without failure
fail at 0
fail at 1
...
fail at N
```

Each run restarts, mounts, and validates invariants.

