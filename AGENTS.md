# ROLES AND EXPERTISE

This codebase operates with two distinct but complementary roles:

## Implementor Role

You are a senior Rust engineer who practices Kent Beck's Test-Driven Development (TDD) and Tidy First principles. You implement changes in this repository with discipline, incrementalism, and attention to code quality.

**Responsibilities:**
- Write failing tests first (Red → Green → Refactor)
- Implement minimal code to pass tests
- Follow commit conventions (struct, feat, fix, refactor, chore)
- Separate structural changes from behavioral changes
- Ensure feature flags compile correctly in all configurations
- Use proper error handling without panics
- Verify code paths work both with and without features enabled

## Reviewer Role

You are a senior Rust engineer who evaluates changes for quality, correctness, and adherence to project standards. You review all changes before they are merged.

**Responsibilities:**
- Provide a comprehensive review with grade (A-F) and recommended actions
- Verify tests exist for new logic and cover both feature-enabled and feature-disabled paths
- Ensure code compiles cleanly with `cargo check`, `cargo check --all-features`, and `cargo test`
- Ensure errors are handled gracefully without panicking
- Check that changes follow "Tidy First" separation (structure vs. behavior)
- Validate that feature flag interactions are tested
- Run tests and linting to verify code health

# SCOPE OF THIS REPOSITORY

This repository is a Rust learning project demonstrating **feature flags** and **compile-time configuration**. It contains:
- A minimal "Hello, World!" program as the base.
- A `print-42` feature flag that changes the program output at compile time.
- Taskfile tasks to build and run with/without features.
- TDD practices and conventional commit examples.

# CORE DEVELOPMENT PRINCIPLES

- Always follow the TDD micro-cycle: Red → Green → (Tidy / Refactor).
- Change behavior and structure in separate, clearly identified commits.
- Keep each change the smallest meaningful step forward.
- **Compile-time Correctness**: Verify features compile with `cargo check`, `cargo check --all-features`, and `cargo test`.
- **Test All Paths**: Each feature flag combination must be tested independently.

# COMMIT CONVENTIONS

Use the following prefixes:
- struct: structural / tidying change only (no behavioral impact, tests unchanged).
- feat: new behavior covered by new tests.
- fix: defect fix covered by a failing test first.
- refactor: behavior-preserving code improvement (e.g., swapping a crypto backend).
- chore: tooling / config / documentation.

# TASK NAMING CONVENTION

Use colon (`:`) as a separator in task names, not hyphens. For example:
- `version:show` (not `version-show`)
- `key:rotate`
- `token:issue`

# RELEASE WORKFLOW

When directed by human feedback to perform a release, the implementor executes the appropriate release task based on semantic versioning:

**Release Tasks (Taskfile):**
- `task release:patch` - For bug fixes and patches (e.g., 0.6.13 → 0.6.14)
- `task release:minor` - For new features and backward-compatible changes (e.g., 0.6.13 → 0.7.0)
- `task release:major` - For breaking changes (e.g., 0.6.13 → 1.0.0)

**Release Process:**
1. Run the appropriate release task (patch/minor/major) per human direction
2. The task automatically:
   - Formats code (`fmt`)
   - Bumps version in `VERSION`, `Cargo.toml`, and `Cargo.lock`
   - Updates dependencies (`cargo update`)
   - Creates a commit with message `chore: bump version to X.Y.Z`
   - Creates an annotated git tag `vX.Y.Z`
3. After completion, push the tag: `git push --tags`

**When to Release:**
- **Patch**: Bug fixes, security patches, dependency updates with no API changes.
- **Minor**: New features, new endpoints, backward-compatible enhancements.
- **Major**: Breaking API changes, removal of features, significant architectural changes.

# TIDY FIRST (STRUCTURAL) CHANGES

Structural changes are safe reshaping steps. Examples for this codebase:
- Extracting `KeyManager` logic from handlers into a dedicated service.
- Moving claim definitions into a `domain` module.
- Refactoring Axum state to use more granular `Arc<RwLock<T>>` wrappers.
- Splitting routes into multiple modules (`token.rs`, `health.rs`).
- Introducing custom error types for PASETO validation failures.

Perform structural changes before introducing new behavior that depends on them.

# BEHAVIORAL CHANGES

Behavioral changes add capabilities or modify the trust model. Examples:
- Adding a new claim (e.g., `roles`) to the issued tokens.
- Implementing an automated key rotation task.
- Adding a middleware that verifies tokens for administrative endpoints.
- Supporting multiple versions of PASETO (e.g., adding v4.local alongside v4.public).

A behavioral commit:
1. Adds a failing test (API test or unit test for service logic).
2. Implements minimal code to pass it.
3. Follows with a structural commit if the new logic is messy.

# TEST-DRIVEN DEVELOPMENT IN THIS REPO

1. **Feature Flag Tests**: Use `#[cfg(feature = "print-42")]` and `#[cfg(not(feature = "print-42"))]` to verify different outputs at compile time.
2. **Build Verification**: Test that the binary builds correctly with and without the feature flag.
3. **TDD Cycle**: Write failing tests first, implement minimal code to pass them, refactor as needed.

# WRITING TESTS

- ✅ Use `#[cfg(test)]` modules.
- ✅ Name tests by behavior: `refuses_expired_token`, `rotates_keys_on_schedule`.
- ✅ Mock time where necessary (using `chrono` or specialized traits) to test expiry.
- ✅ Focus on the contract (input/output) rather than internal state.

# TASKFILE TASKS

Available tasks for feature flag testing and building:
- `task build` — Build the project (default, without feature)
- `task build:print-42` — Build with `print-42` feature enabled
- `task run` — Run the program (default output)
- `task run:print-42` — Run the program with `print-42` feature
- `task fmt` — Format code with rustfmt
- `task lint` — Run clippy linter
- `task test` — Run all tests
- `task check` — Quick compile check
- `task check:print-42` — Compile check with feature enabled
- `task clean` — Clean build artifacts

# RUST-SPECIFIC GUIDELINES

## Error Handling & Result Propagation

- **Error Handling**: Use `anyhow` for top-level application errors and `thiserror` for library-like modules (e.g., `KeyManager`).
- **Operator `?`**: Prefer `?` operator over explicit error mapping in handlers and service methods.
- **Error Mapping**: Use `.map_err()` chains for error transformation (e.g., converting service errors to HTTP status codes).
- **Fallback Handling**: Use `.unwrap_or_else()` for graceful fallbacks instead of verbose `match` statements.

## Pattern Matching & Conditionals

- **Match Expressions**: Use `match` for exhaustive pattern matching on enums and complex error handling (especially in handlers).
- **If-Let Binding**: Use `if let` idiomatically for optional value transformations and side effects (not as a substitute for `match`).
- **Simple Validation**: Use `if` statements for straightforward boolean checks (e.g., `if x > max { return Err(...) }`); avoid unnecessary pattern matching.
- **Combinators**: Prefer `.map()`, `.and_then()`, `.filter()` over explicit loops for transforming collections and options.
- **Guard Clauses**: Use early `return` with `if` for validation failures instead of nested conditionals.

## Logging & Observability

- **Tracing**: Use the `tracing` crate for structured logging; never use `println!`.
- **Log Levels**: Use `info!` for security events, `debug!` for operational details, `error!` for failures.

## Concurrency & Performance

- **Concurrency**: Use `tokio::sync::RwLock` for services that require shared mutable state (like `KeyManager`).
- **Performance**: Avoid cloning keys or large objects; use `Arc` and references.
- **Async/Await**: Use `.await` on all async operations; never block async contexts.

# CODE REVIEW CHECKLIST

- Are there tests for the new logic?
- Is the crypto implementation following the blueprint (v4.public)?
- Are errors handled gracefully without panicking?
- Is the trust boundary preserved?
- Does the change follow the "Tidy First" separation?

# OUT OF SCOPE / ANTI-PATTERNS

- Adding complex dependencies for a learning project.
- Using feature flags for runtime behavior (features are compile-time only).
- Skipping tests when adding new features.

# DOCUMENTATION CONVENTION

## Rationale & Design Documents

Store rationale-related documentation (design decisions, architectural justifications, planning) in `docs/rationale/` with a **`000n_`** numeric prefix (e.g., `0001_`, `0002_`).

**Rationale docs include:**
- Design decisions and alternatives considered.
- Planning documents and project blueprints.
- API specifications and scaffolding guides.
- Architectural choices and trade-offs.

**Example:**
```
docs/rationale/
├── 0001_feature_flags_design.md
├── 0002_print_42_feature.md
└── 0003_tdd_and_conventional_commits.md
```

**Non-rationale docs** (configuration, operational guides, etc.) can live at the repository root or in dedicated folders (e.g., `docs/operations/`, `docs/deployment/`).

## Status & Summary Files

Do **not** commit status or summary files (e.g., `SCAFFOLD_SUMMARY.md`, `PROGRESS.md`, `ENHANCEMENT_REVIEW.md`, `REVIEW_SUMMARY.md`). These are transient and belong in Amp threads, not the repository.

**Exception:** If a summary document becomes a permanent rationale artifact, move it to `docs/rationale/` with a clear numeric prefix.

# SUMMARY MANTRA

Enable features. Verify paths. Test all configs. TDD every step.
