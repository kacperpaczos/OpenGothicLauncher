# Architecture Overview

The OpenGothicLauncher architecture is based on **Clean Architecture** principles, specifically the **Ports and Adapters** (Hexagonal) pattern. This design ensures that the core business logic is isolated from external side effects and infrastructure details.

## Structural Logic
The system is divided into four conceptual layers:

1.  **Domain (Innerest)**: Contains entities and pure business rules. It has zero dependencies on other modules.
2.  **Ports (Interface Layer)**: Defines the boundaries. This is where traits reside that describe *what* the system needs to do (e.g., "save config", "download engine").
3.  **Services (Application Layer)**: Orchestrates the flow of data between the domain and the ports. It contains the "brains" of the application like `LauncherService`.
4.  **Adapters (Infrastructure/UI Layer)**: Concrete implementations of ports and user interfaces. This includes `ogl-infra` (filesystem), `ogl-network` (HTTP), and `ogl-gui` (UI).

## Key Principles
- **Dependency Inversion**: High-level modules (Services) do not depend on low-level modules (Adapters). Both depend on abstractions (Ports).
- **Single Source of Truth**: The `ogl-core` crate is the unique owner of the application's logical state and rules.
- **Platform Agnosticism**: All platform-specific code is isolated in `ogl-infra`, keeping the rest of the codebase pure.
