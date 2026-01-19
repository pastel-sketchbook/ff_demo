# TDD and Conventional Commits

**Date**: 2026-01-19  
**Status**: Active  
**Related**: `0001_feature_flags_design.md`

## Overview

This project demonstrates Test-Driven Development (TDD) and conventional commits as applied to feature flag implementation in Rust.

## TDD Micro-Cycle

### Red → Green → Refactor

1. **Red**: Write a failing test for the new behavior (e.g., "when feature X is enabled, output Y")
2. **Green**: Implement minimal code to make the test pass
3. **Refactor**: Improve code structure without changing behavior (optional for this simple project)

### Feature Flag Testing

For feature flags, the TDD cycle includes verification that:
- Code compiles with the feature enabled
- Code compiles with the feature disabled
- Tests pass in both configurations

Example workflow:
```bash
# Red: feature does not compile yet
cargo test --features new-feature  # fails

# Green: implement minimal code
# ... add code with #[cfg(feature = "new-feature")]

cargo test --features new-feature   # passes
cargo test                           # passes (default, no feature)

# Verify both paths work
cargo check --all-features           # compiles
cargo check                          # compiles
```

## Conventional Commits

This project uses conventional commit messages to document intent and type of change.

### Commit Prefixes

| Prefix | Purpose | Example |
|--------|---------|---------|
| `feat:` | New behavior, covered by tests | `feat: add print-42 feature` |
| `fix:` | Bug fix, covered by failing test first | `fix: print-42 output trimmed` |
| `struct:` | Structural change, no behavior change | `struct: extract constants module` |
| `refactor:` | Code improvement, behavior-preserving | `refactor: simplify cfg conditions` |
| `chore:` | Tooling, config, documentation | `chore: update README` |

### Tidy First

Separate structural changes from behavioral changes:

**Structural commit** (no behavior change):
```
struct: move feature flags to constants

- Extract print-42 flag to consts
- No behavior change, tests unchanged
```

**Behavioral commit** (includes tests):
```
feat: add print-1337 feature flag

- New conditional code path
- New test to verify output
- Feature gated in Cargo.toml
```

## Why This Matters

1. **Clarity**: Future developers understand *why* each change was made
2. **Traceability**: `git log --oneline --grep="feat:"` finds all feature additions
3. **Discipline**: Separating structure from behavior forces thoughtful change design
4. **Revertability**: Tidy-first commits can be reverted independently if needed

## Scope of This Project

This project is intentionally minimal to keep focus on TDD and commit discipline:
- Single file (`main.rs`)
- One feature (`print-42`)
- Basic output

As students master these concepts, they can apply them to:
- Multi-file projects
- Multiple features
- Dependency management
- Complex logic gated by features
