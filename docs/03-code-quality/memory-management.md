# Memory Management

The project leverages Rust's ownership model to ensure memory safety without a garbage collector.

## Strategies
1.  **Arc for Resource Sharing**: `Arc` (Atomic Reference Counting) is used to share services and state between the main UI thread and Tokio background tasks.
2.  **Stack over Heap**: Data structures are kept compact, favoring stack allocation where possible.
3.  **Zero-copy (where possible)**: The system avoids unnecessary string/struct cloning by passing references or moving ownership.
4.  **RAII (Resource Acquisition Is Initialization)**: Files and network connections are automatically closed when their handles go out of scope.

## Observations
- No `unsafe` blocks were identified in the internal crates, relying instead on the standard library and audited community crates for low-level tasks.
- Static assets (icons) are bundled into the binary using `include_bytes!`, which maps them directly to the executable's read-only memory section.
