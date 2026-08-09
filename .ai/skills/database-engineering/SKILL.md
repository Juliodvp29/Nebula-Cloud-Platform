# Database Engineering

## Purpose

This skill defines database engineering rules for Nebula’s PostgreSQL
control plane.

PostgreSQL is the authoritative source of control-plane state. MinIO
stores object data; PostgreSQL stores the metadata and state required to
manage that data.

## Design authority

Database changes must follow:

1.  `DATABASE_DESIGN.md`
2.  `schema.sql`
3.  approved architectural decisions documented in the project

Do not introduce tables or relationships solely because they seem
useful.

## PostgreSQL schemas

Use PostgreSQL schemas to establish clear ownership boundaries.

Keep related tables grouped by domain and avoid placing the entire
system into one undifferentiated namespace.

A table should have a clear owner.

## Identity and IDs

Nebula uses UUIDs and cloud-facing resource IDs according to the
database design.

Distinguish between:

- internal database identity;
- public/cloud resource identity.

Do not expose internal identifiers merely because they are available.

## Tenancy and ownership

Every resource must have an explicit ownership/tenant model where
applicable.

Authorization must not depend solely on application code when a database
constraint can safely enforce an invariant.

Consider:

- organization/project ownership;
- resource ownership;
- region;
- resource type;
- lifecycle state.

## Data types

Prefer explicit PostgreSQL types and constraints.

For controlled string values, use the project’s chosen `VARCHAR + CHECK`
strategy rather than silently introducing PostgreSQL enums.

Use `NOT NULL` when required, `CHECK` constraints for bounded values,
foreign keys for relationships, and unique constraints for true
uniqueness.

## Soft deletion and lifecycle

Nebula uses soft deletion/lifecycle semantics where defined.

Do not physically delete records when the domain requires historical
traceability.

Queries must intentionally decide whether deleted/inactive resources are
included.

Lifecycle state transitions should be explicit rather than arbitrary
string updates.

## Resource registry

The Resource Registry is a cross-service concept.

Resources should have consistent identity and lifecycle metadata without
forcing every domain into a single giant table design.

When adding a resource type, consider how it participates in:

- resource identity;
- region;
- ownership;
- authorization;
- quotas;
- usage;
- auditing;
- lifecycle.

## S3/storage data model

Object storage metadata belongs in PostgreSQL while object bytes are
handled by MinIO.

The database model must account for:

- buckets;
- objects;
- object versions;
- metadata;
- multipart uploads;
- lifecycle;
- ownership;
- authorization;
- presigned URL related state where required.

Do not treat MinIO as the system of record for Nebula’s control-plane
metadata.

## EC2/compute data model

Compute resources must represent state independently from the Docker
runtime.

Database state should be capable of describing desired/known resource
state even when Docker is temporarily unavailable.

Docker is an infrastructure adapter, not the source of truth for
Nebula’s resource model.

Volumes are represented as Docker volumes according to the project
decision.

Images reference external Docker images rather than requiring Nebula to
become an image registry.

## Lambda/functions data model

Functions support:

- archive-based deployment;
- container-image deployment;
- Node.js;
- Python;
- custom runtime;
- execution roles;
- versions/invocations as defined by the design.

Deployment and execution state should be represented independently from
the runtime process.

## Jobs and asynchronous operations

Long-running operations should be represented as durable jobs.

Redis may coordinate queues/workers, but PostgreSQL should preserve
durable state that must survive Redis failure.

Design jobs with:

- status;
- attempts;
- timestamps;
- error information;
- idempotency where applicable;
- ownership/resource references.

## Idempotency

Mutating API operations that can be retried should use idempotency keys
where defined.

Database constraints should support idempotency guarantees.

Never assume a client sends a request only once.

Consider failures between:

1.  database commit;
2.  external infrastructure operation;
3.  response delivery.

## Quotas and usage

Quotas should be enforceable and resistant to race conditions.

Usage events should be append-oriented where appropriate and should not
be confused with current resource state.

When introducing usage accounting, define whether the value is:

- authoritative state;
- derived state;
- an event;
- a cached value.

## Audit logs

Security-sensitive and resource-changing operations should generate
audit information according to the project’s audit design.

Audit records should remain useful even when the original resource
changes later.

Do not store secrets in audit logs.

## Indexing

Create indexes based on actual access patterns.

Before adding an index, identify query/filter columns, sort order,
cardinality, expected table size and uniqueness requirements.

Avoid indexing every column by default.

## Transactions

Use transactions for operations whose invariants span multiple database
changes.

Keep transactions focused and short.

Do not hold database transactions open while waiting on slow external
systems such as Docker or MinIO unless there is a deliberate reason.

For database + external-system workflows, prefer explicit state
machines, jobs, idempotency and reconciliation.

## Migrations

Schema evolution must be migration-friendly.

Never assume a production database can be destroyed and recreated.

Prefer additive, backwards-compatible changes where possible.

When changing a column or constraint, consider existing rows and
deployed application versions.

## SQL quality

Prefer explicit column lists.

Avoid `SELECT *` in application queries.

Use parameterized queries.

Do not construct SQL using string concatenation with user-controlled
values.

Keep database-specific logic in infrastructure/repository layers rather
than HTTP handlers.

## Review checklist

Before accepting a database change, verify:

- [ ] ownership/tenant isolation
- [ ] correct schema
- [ ] correct ID strategy
- [ ] constraints
- [ ] foreign keys
- [ ] indexes
- [ ] lifecycle/soft delete
- [ ] idempotency
- [ ] audit/usage implications
- [ ] transaction boundaries
- [ ] migration strategy
- [ ] failure behavior
