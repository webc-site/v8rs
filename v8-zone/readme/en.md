# v8-zone

Rust port of V8 `src/zone` (arena allocator), L0 foundation layer. `Zone` bump allocation with segment chaining, `AccountingAllocator` tracing, and zone containers (`ZoneVector`, `ZoneList`, `ZoneDeque`/`ZoneQueue`/`ZoneStack`, `ZoneForwardList`/`ZoneLinkedList`, `ZoneAllocator`/`RecyclingZoneAllocator`). Lifetimes replace raw `Zone*`: allocations and containers borrow the zone via `'z`.

Containers also include `ZoneCompactSet` (zone-compact-set.h port: packed pointer+2-bit-tag empty/singleton encoding, copy-on-write sorted zone list). Under the `precise-zone-stats` feature, `TypeStats` (type-stats.h port) records per-TypeTag allocation ledgers, merged and dumped by the tracing `AccountingAllocator` (accounting-allocator.cc:162-175).

See [中文文档](./zh.md) for the full module/test mapping tables, the bumpalo kernel evaluation (not adopted), the semantic deviation log, the deferred list, and the symbol audit confirming that `AccountingScope`/`AllocationRecord`/three-mode allocation/`ZoneUnique`/`MakeZoned` do not exist in this V8 checkout.

Development log (adversarial review round fix registers): see ../../log/v8-zone.md.
