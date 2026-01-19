# Rust Feature Flags Learning Project

A minimal Rust learning project demonstrating **compile-time feature flags** and configuration. Learn how to enable features, set opt-in defaults, and conditionally manage dependencies.

## Project Goals

This project teaches:

- How to **enable** feature flags and build with them
- Setting feature defaults as **opt-in** (disabled by default)
- Conditionally including **dependencies** based on features
- Using `#[cfg(feature = "...")]` for compile-time code selection
- Testing multiple code paths with different feature combinations
- TDD practices and conventional commits for feature development

## Quick Start

### Build (default, no features)
```bash
task build
cargo run
# Output: Hello, world!
```

### Build with print-42 Feature
```bash
task build:print-42
cargo run --features print-42
# Output: 42
```

### Build with lucky-number Feature (Conditional Dependency)
```bash
task build:lucky-number
cargo run --features lucky-number
# Output: Hello, world!
#         Your lucky number: 42
```

### Build with All Features
```bash
task build:all-features
cargo run --all-features
# Output: 42
#         Your lucky number: 95
```

## Available Tasks

| Task | Description |
|------|-------------|
| `task build` | Build without features |
| `task build:print-42` | Build with `print-42` feature |
| `task build:lucky-number` | Build with `lucky-number` feature |
| `task build:all-features` | Build with all features |
| `task run` | Run default output |
| `task run:print-42` | Run with `print-42` feature |
| `task run:lucky-number` | Run with `lucky-number` feature |
| `task run:all-features` | Run with all features |
| `task check` | Quick compile check (no features) |
| `task check:print-42` | Quick compile check with `print-42` |
| `task check:lucky-number` | Quick compile check with `lucky-number` |
| `task check:all-features` | Quick compile check with all features |
| `task fmt` | Format code |
| `task lint` | Run clippy linter |
| `task test` | Run all tests |
| `task clean` | Clean build artifacts |

## How Feature Flags Work

### 1. Declare Features in Cargo.toml

Features are explicitly listed in the `[features]` section. By default, they are **opt-in** (disabled):

```toml
[features]
print-42 = []
```

This declares `print-42` as an optional feature. Users must explicitly enable it:

```bash
cargo build --features print-42
```

### 2. Conditional Compilation with #[cfg]

Use `#[cfg(feature = "...")]` and `#[cfg(not(feature = "..."))]` to gate code at compile time:

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

### 3. Conditional Dependencies

You can also gate dependencies in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", optional = true }

[features]
json-support = ["serde"]
```

When `json-support` is enabled, `serde` is compiled and available. When disabled, it's not compiled at all.

### Key Principles

- **Compile-time**: Features are resolved during compilation; there's no runtime overhead.
- **No runtime equivalent**: Unlike environment variables, feature flags cannot be changed at runtime.
- **Mutually exclusive defaults**: A feature is **disabled by default** unless explicitly enabled by the user or listed as a default.

## Real-World Example: The `lucky-number` Feature

This project includes a practical example: the `lucky-number` feature that pulls in the `rand` crate.

### The Feature Setup

In `Cargo.toml`:
```toml
[dependencies]
rand = { version = "0.8", optional = true }

[features]
lucky-number = ["rand"]
```

The `rand` crate is marked `optional = true`, so it's not compiled by default. The `lucky-number` feature explicitly includes `rand` in its dependency list.

### Code Implementation

```rust
#[cfg(feature = "lucky-number")]
fn generate_lucky_number() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}
```

Only when the `lucky-number` feature is enabled does this function (and the `rand` crate) get compiled.

### Binary Size Impact

- **Without feature**: ~2.8 MB (lean, no `rand`)
- **With feature**: ~3.2 MB (includes `rand` and dependencies)

By keeping heavy dependencies optional, you let users build exactly what they need.

### Testing Both Paths

Run tasks to verify the feature works in all configurations:

```bash
# Test without the feature
task check              # compiles
task run               # outputs "Hello, world!"

# Test with the feature
task check:lucky-number # compiles
task run:lucky-number  # outputs "Hello, world!\nYour lucky number: 42"

# Test all features together
task check:all-features # compiles all feature combinations
task run:all-features   # runs with all enabled
```

## Development Workflow

This project follows **Test-Driven Development (TDD)** with conventional commits:

1. **Red**: Write failing tests first
2. **Green**: Implement minimal code to pass tests
3. **Refactor**: Clean up (if needed)

### Commit Types
- `feat:` New behavior with tests
- `fix:` Bug fixes with tests
- `struct:` Structural/tidy changes (no behavior change)
- `refactor:` Code improvements (behavior-preserving)
- `chore:` Tooling, config, documentation

## Testing Both Paths

Run tests to verify both code paths compile:

```bash
task check          # Verify default path compiles
task check:print-42 # Verify feature path compiles
task test           # Run all tests
```

## Best Practices

| Practice | Example |
|----------|---------|
| **Opt-in by default** | Features start disabled; users enable what they need |
| **Name clearly** | Use descriptive names: `json-support`, `async-runtime`, not `feat1` |
| **Gate dependencies** | Only compile optional dependencies when their feature is enabled |
| **Test all paths** | Run `task check` and `task check:print-42` to verify both compile |
| **Document in code** | Use comments to explain what each `#[cfg(...)]` branch does |
| **Avoid feature creep** | Keep features focused; don't create 10 tiny flags for one subsystem |

## Common Patterns

### Pattern 1: Optional Feature (Disabled by Default)
```toml
[features]
advanced = []
```

### Pattern 2: Feature with Dependencies
```toml
[dependencies]
tokio = { version = "1", optional = true }

[features]
async = ["tokio"]
```

### Pattern 3: Multiple Features
```toml
[features]
default = []                    # Empty means all features are opt-in
logging = []                    # Disabled by default
metrics = []                    # Disabled by default
all = ["logging", "metrics"]    # Convenience feature that enables all
```

## References

- [Rust: Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [Cargo: Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [AGENTS.md](./AGENTS.md) — Project conventions and guidelines
