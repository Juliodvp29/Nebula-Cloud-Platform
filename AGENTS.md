# AGENTS.md

## Project

Nebula is an educational cloud infrastructure platform designed with a
serious, production-oriented architecture.

The project is strongly inspired by AWS concepts without attempting to
reproduce AWS APIs literally.

Core services:

- IAM — identity and access control
- Storage — object storage inspired by S3
- Compute — container-based compute inspired by EC2
- Functions — serverless functions inspired by Lambda

Current architectural direction:

- Modular monolith first
- Rust backend
- PostgreSQL as the control-plane source of truth
- Redis for queues, jobs, caching and coordination where appropriate
- MinIO as the object-storage engine
- Docker as the compute/runtime engine
- Angular frontend later
- Public HTTP API independent from the Angular frontend
- Simulated regions from the beginning
- Networking outside the MVP and explicitly reserved for future work

## Non-negotiable architecture rules

1.  Do not introduce microservices unless explicitly decided.
2.  `main.rs` is responsible for application composition, not business
    logic.
3.  HTTP handlers must remain thin.
4.  Business rules belong in application/domain layers, not HTTP
    handlers.
5.  Infrastructure concerns must not leak into domain logic.
6.  Do not access PostgreSQL, Redis, MinIO or Docker directly from HTTP
    handlers.
7.  Shared infrastructure is injected through application state/services
    rather than created inside handlers.
8.  Avoid circular dependencies between domain modules.
9.  Prefer explicit interfaces/traits when they create a meaningful
    architectural boundary.
10. Do not introduce abstractions merely for theoretical purity.
11. Important external operations must account for partial failure,
    retries and idempotency.
12. Security-sensitive behavior must fail closed.
13. Never commit secrets, credentials or local `.env` files.
14. Do not silently change architectural decisions documented in
    `PROJECT.md`, `DATABASE_DESIGN.md` or `BACKEND_ROADMAP.md`.

## Rust rules

- Prefer idiomatic Rust.
- Avoid `unwrap()` and `expect()` in production paths unless the
  invariant is truly guaranteed and documented.
- Prefer typed errors with `thiserror` for application/library errors.
- Preserve ownership boundaries deliberately; avoid unnecessary cloning.
- Keep async code explicit and avoid blocking operations inside async
  request paths.
- Run `cargo fmt`, `cargo check` and `cargo clippy` after meaningful
  changes.
- Do not suppress warnings globally just to obtain a clean build.

## Database rules

- PostgreSQL is the source of truth for control-plane state.
- Follow `DATABASE_DESIGN.md` and `schema.sql`.
- Use UUIDs/cloud IDs as defined by the database design.
- Preserve tenant/resource isolation.
- Mutating operations should be designed for idempotency where
  appropriate.
- Do not bypass the repository/application boundary with ad-hoc SQL from
  handlers.
- Schema changes must be deliberate and migration-friendly.
- Soft deletion and lifecycle semantics must be respected where defined.

## Cloud resource rules

When implementing a resource, consider:

- owner/tenant
- region
- resource identity
- status/state
- timestamps
- soft deletion/lifecycle
- authorization
- quotas
- usage events
- audit events
- idempotency
- asynchronous jobs where operations may be long-running

## AI-assisted development workflow

Before changing code:

1.  Read the relevant project documentation.
2.  Inspect the existing implementation.
3.  Search for existing abstractions before creating new ones.
4.  Identify which architectural boundary the change belongs to.
5.  Make the smallest coherent change.
6.  Run the relevant tests/checks.
7.  Explain architectural changes separately from implementation
    details.

Do not:

- invent APIs or modules that are not required;
- rewrite unrelated code;
- duplicate existing services;
- add dependencies without justification;
- change the database schema casually;
- turn a small feature into a broad refactor.

## Documentation hierarchy

Use these files as the primary project references:

1.  `PROJECT.md` — project vision, scope and architecture
2.  `BACKEND_ROADMAP.md` — implementation sequence
3.  `DATABASE_DESIGN.md` — database architecture and decisions
4.  `schema.sql` — current database schema
5.  `.ai/skills/*/SKILL.md` — specialized engineering guidance

When implementation and documentation disagree, stop and surface the
discrepancy instead of silently choosing one.

## Definition of done

A change is not considered complete merely because it compiles.

Depending on the change, completion may require:

- formatting
- linting
- unit/integration tests
- database migration/schema updates
- audit/usage behavior
- authorization checks
- idempotency handling
- documentation updates
- verification of failure paths

Keep the project educational, but do not use “educational” as an excuse
for careless architecture.
