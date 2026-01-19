# Print-42 Feature

**Date**: 2026-01-19  
**Status**: Active  
**Related**: `0001_feature_flags_design.md`

## Overview

The `print-42` feature is a minimal, concrete example of compile-time feature flags in action. It demonstrates how the same binary can have different behavior based on a feature flag selection at build time.

## Design

### Feature Declaration

```toml
[features]
print-42 = []
```

- No dependencies required
- Purely demonstrates conditional compilation

### Implementation

Two code paths in `main.rs`:

```rust
#[cfg(feature = "print-42")]
fn main() {
    println!("42");
}

#[cfg(not(feature = "print-42"))]
fn main() {
    println!("Hello, world!");
}
```

## Pedagogical Value

1. **Simplicity**: Single function, two output strings—no complexity to obscure the feature mechanism
2. **Immediate Feedback**: Students can see the difference immediately by running:
   - `cargo run` → "Hello, world!"
   - `cargo run --features print-42` → "42"
3. **Foundation**: Establishes the pattern for conditional code before moving to dependencies

## Testing Both Paths

The Taskfile provides tasks to verify both paths compile and run:

- `task run` — Default (no feature)
- `task run:print-42` — Feature enabled
- `task check` — Verify default path compiles
- `task check:print-42` — Verify feature path compiles

## Why "42"?

Reference to Douglas Adams' "The Hitchhiker's Guide to the Galaxy" (the answer to life, the universe, and everything). A symbolic and memorable output that makes the behavioral difference obvious.

## Future Extensions

Students can extend this concept by:
1. Adding a `print-1337` feature
2. Creating a feature that gates a dependency (e.g., `serde`)
3. Testing multiple feature combinations
4. Creating mutually exclusive features
