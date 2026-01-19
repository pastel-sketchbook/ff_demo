# Feature Flags Design

**Date**: 2026-01-19  
**Status**: Active

## Overview

This project demonstrates Rust feature flags as a compile-time configuration mechanism. Feature flags allow different code paths and dependencies to be selected at build time, not runtime.

## Design Decision: Opt-In Default

**Decision**: All features are opt-in (disabled by default).

**Rationale**:
- Mirrors production best practice: minimize attack surface and binary bloat by including only what's needed
- Teaches users to explicitly enable features they require
- Prevents accidental feature inclusion
- Aligns with Cargo's default behavior

**Alternative Considered**: Opt-out (features enabled by default)
- Would increase default binary size
- Less pedagogically valuable for learning zero-configuration principles

## Conditional Dependencies

**Decision**: Features can gate optional dependencies.

**Example**:
```toml
[dependencies]
serde = { version = "1.0", optional = true }

[features]
json-support = ["serde"]
```

**Rationale**:
- Demonstrates real-world use case: only compile dependencies when their feature is enabled
- Reduces build time and binary size when features are not needed
- Teaches students how to structure feature-gated codebase architecture

## Compile-Time vs. Runtime

**Decision**: Use only compile-time feature flags; no runtime configuration.

**Rationale**:
- Feature flags are resolved during compilation, not at runtime
- Unlike environment variables or config files, feature flags are baked into the binary
- Enables dead code elimination and zero-cost abstractions
- Teaches students that this is a build-time concern, not a runtime concern

## Testing Strategy

**Decision**: Test all feature combinations independently.

**Rationale**:
- Each feature combination must compile successfully
- Running `cargo check`, `cargo check --all-features`, and `cargo test` ensures correctness
- Prevents accidental breakage in disabled code paths

## Out of Scope

This project intentionally excludes:
- Cryptographic operations (security-focused projects)
- Runtime configuration (environment variables, config files)
- Complex feature interactions (for simplicity)
- Production patterns (for pedagogical focus)
