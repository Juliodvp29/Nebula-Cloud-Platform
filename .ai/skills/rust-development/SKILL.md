# Rust Development

## Principles

- Prefer idiomatic Rust.
- Do not use unwrap() in production paths.
- Prefer typed errors.
- Avoid unnecessary cloning.
- Prefer borrowing when ownership does not need to transfer.
- Keep async boundaries explicit.
- Do not introduce Arc/Mutex without a concrete ownership/concurrency reason.

## Before implementing

1. Understand the existing module.
2. Search for existing abstractions.
3. Check whether the functionality belongs to the domain,
   application or infrastructure layer.
4. Check existing tests.

## Dependencies

Before introducing a crate:

1. Check whether the functionality already exists.
2. Evaluate whether the dependency is justified.
3. Prefer mature, maintained crates.
4. Avoid dependencies that introduce unnecessary complexity.
