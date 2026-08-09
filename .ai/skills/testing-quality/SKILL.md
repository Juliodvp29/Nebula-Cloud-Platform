# Testing & Quality

## Purpose

This skill defines the testing and quality standards for Nebula.

The goal is not maximum test count. The goal is confidence in business
rules, security boundaries, infrastructure behavior and failure
handling.

## Quality gates

Meaningful Rust changes should normally pass:

```bash
cargo fmt -- --check
cargo check
cargo clippy
cargo test
```

Do not suppress warnings simply to make the pipeline green.

## Test pyramid

Prefer a layered strategy:

1.  Unit tests for domain rules and pure logic.
2.  Application/service tests for use cases and orchestration.
3.  Integration tests for PostgreSQL, Redis, MinIO and other real
    infrastructure where behavior depends on the integration.
4.  API tests for HTTP contracts and authorization.
5.  End-to-end tests for critical user flows.

Do not turn every test into an end-to-end test.

## Unit tests

Unit tests should be fast and deterministic.

Prioritize:

- authorization decisions;
- policy evaluation;
- resource state transitions;
- validation;
- quota calculations;
- idempotency logic;
- serialization rules;
- pure domain behavior.

Avoid requiring Docker, PostgreSQL or MinIO for tests that can be
expressed as pure unit tests.

## Application tests

Application-layer tests should verify orchestration and business use
cases.

Examples:

- creating a bucket;
- attaching a role;
- launching a compute resource;
- deploying a function;
- starting a job;
- enforcing a quota.

Use controlled test doubles when the external system is not the behavior
under test.

## Integration tests

Use real infrastructure when correctness depends on the integration.

Important integration targets include:

- PostgreSQL queries and constraints;
- Redis queue/coordination behavior;
- MinIO object operations;
- Docker lifecycle operations.

Prefer disposable test infrastructure rather than mocks that merely
reproduce expected behavior.

## API tests

API tests should verify:

- status codes;
- response structure;
- validation;
- authentication;
- authorization;
- tenant isolation;
- idempotency;
- error behavior.

Every protected endpoint should have tests for both allowed and denied
access.

## Failure-path testing

Nebula is an infrastructure platform, so failure paths are first-class
behavior.

Where relevant, test scenarios such as:

- PostgreSQL unavailable;
- Redis unavailable;
- MinIO unavailable;
- Docker unavailable;
- external operation times out;
- request is retried;
- worker crashes;
- operation succeeds externally but database update fails;
- database update succeeds but response is lost;
- duplicate idempotency key;
- unauthorized resource access.

## State-machine testing

Resources such as compute instances, jobs and deployments have lifecycle
states.

Test valid transitions and reject invalid ones.

For example:

```text
PENDING
  ↓
PROVISIONING
  ↓
RUNNING
  ↓
STOPPING
  ↓
STOPPED
```

Do not allow arbitrary state changes merely because the database column
accepts a string.

## Security testing

Security tests are mandatory for:

- IAM policy evaluation;
- role inheritance/attachment;
- resource ownership;
- tenant isolation;
- credential handling;
- presigned URLs;
- execution roles;
- protected API routes.

Tests should prove that unauthorized access is rejected.

## Idempotency testing

For idempotent operations, test:

1.  first request;
2.  identical retry;
3.  same key with different payload;
4.  failure followed by retry;
5.  concurrent duplicate requests where relevant.

The expected result must be explicitly defined.

## Database testing

Database tests should verify constraints that are part of the business
invariant.

Do not rely exclusively on application-level validation if PostgreSQL
can enforce the invariant safely.

Test:

- foreign keys;
- uniqueness;
- check constraints;
- soft-delete behavior;
- lifecycle transitions;
- transaction behavior.

## Determinism

Tests should avoid:

- dependence on wall-clock timing where unnecessary;
- random values without controlled seeds;
- shared mutable global state;
- test ordering assumptions.

When time matters, prefer injectable clocks or controlled timestamps.

## Test naming

Test names should explain behavior.

Prefer:

```text
denies_object_access_when_user_lacks_permission
```

over:

```text
test_1
```

## What not to test

Do not write tests solely to increase coverage.

Avoid brittle tests that verify:

- internal implementation details;
- exact private function structure;
- incidental log formatting;
- dependency internals.

Test observable behavior and important invariants.

## Definition of done

A feature is not complete when the happy path works.

For meaningful features, verify:

- happy path;
- validation failure;
- authorization failure;
- duplicate/retry behavior;
- relevant infrastructure failure;
- lifecycle/state behavior;
- audit/usage side effects where applicable.

Document intentional test gaps when a required integration cannot yet be
automated.
