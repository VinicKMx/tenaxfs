# Performance

TenaxFS does not aim to be the fastest filesystem for every workload. Its main
performance claim is explicit control and observability over maintenance
interference.

Benchmarks should measure distributions, not only averages:

- p50;
- p95;
- p99;
- maximum.

Metrics should include:

- mount time;
- read latency;
- put latency;
- transaction commit latency;
- stream append latency;
- maintenance cost;
- GC copy cost;
- erase interference;
- write amplification;
- RAM use;
- flash metadata overhead.

Storage conditions should include empty, partially full, nearly full,
fragmented, active GC, high wear imbalance, and active candidate epoch states.

