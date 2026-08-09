# Backend Architecture

Nebula is a modular monolith.

Do not introduce microservices unless explicitly requested.

Dependencies should flow:

API
↓
Application
↓
Domain
↓
Infrastructure

Domain code must not depend on infrastructure implementations.

HTTP handlers must not contain business logic.

Repositories must not contain business rules.

Infrastructure adapters must implement domain/application interfaces.
