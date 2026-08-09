# AI Development Workflow

## Purpose

This skill defines how AI should work on Nebula.

The goal is to use AI for acceleration without allowing generated code
to erode the architecture, security model or maintainability of the
project.

## Before implementation

Always:

1.  Read the relevant `PROJECT.md`, `BACKEND_ROADMAP.md` and
    `DATABASE_DESIGN.md` sections.
2.  Inspect the current module structure and existing implementation.
3.  Search for existing types, services, repositories and utilities.
4.  Identify the correct architectural layer.
5.  Determine whether the task changes an existing architectural
    decision.
6.  State important assumptions before making a large change.

Do not start by generating a large amount of code.

## Implementation strategy

Prefer:

- small, reviewable changes;
- one architectural concern at a time;
- existing project conventions;
- explicit types;
- typed errors;
- deterministic behavior;
- testable application logic;
- minimal dependencies.

Avoid:

- speculative abstractions;
- unnecessary generic frameworks;
- large rewrites;
- duplicate implementations;
- generated boilerplate with no current purpose.

## Repository awareness

Before adding a new module, service, trait, dependency, database table,
Redis key, background job or API endpoint, check whether an equivalent
already exists.

If an existing abstraction is insufficient, prefer extending it over
creating a parallel abstraction.

## Architecture review

For every meaningful backend change, ask:

- Does this belong to API, application, domain or infrastructure?
- Does it create a new dependency direction?
- Does it introduce coupling between cloud services?
- Does it affect resource ownership or tenant isolation?
- Does it require authorization?
- Does it need an audit event?
- Does it affect quotas or usage?
- Can it partially fail?
- Does it need idempotency?
- Should it be synchronous or a background job?

## Database changes

AI must not casually modify `schema.sql`.

For database work:

1.  Compare the requested change with `DATABASE_DESIGN.md`.
2.  Check existing tables and constraints.
3.  Consider foreign keys, indexes, uniqueness and lifecycle semantics.
4.  Consider tenant/resource isolation.
5.  Consider migrations and backward compatibility.
6.  Update documentation when an architectural decision changes.

## Security

Treat authentication, authorization, credentials, presigned URLs and
secrets as security-sensitive.

Never:

- log secrets;
- expose credentials in API responses;
- bypass IAM for convenience;
- trust client-provided ownership;
- weaken authorization to make tests pass.

Security behavior should fail closed.

## Validation

After implementation, run the smallest useful verification set,
normally:

```bash
cargo fmt -- --check
cargo check
cargo clippy
cargo test
```

For database changes, also validate migrations/schema.

For API changes, test both successful and rejected requests.

## Handling uncertainty

If the existing architecture does not clearly determine the correct
implementation:

1.  Identify the ambiguity.
2.  Present the relevant options.
3.  Prefer the option most consistent with existing project decisions.
4.  Do not silently invent a new architectural rule.

## AI output quality

Generated code must be understandable by a human Rust developer.

Do not optimize for code volume.

A smaller implementation that respects Nebula’s architecture is
preferable to a larger implementation that merely works.
