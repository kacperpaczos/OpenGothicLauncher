# Branching Strategy

The project uses a structured branching model to maintain stability while allowing rapid feature development.

## Standard Branches
- **`main`**: The stable branch. Contains production-ready code.
- **`develop`**: The integration branch (if used).

## Feature Branches
- **Naming Convention**: `<username>/feat/<feature-name>` (e.g., `kacperpaczos/feat/rearchitecture`).
- **Lifecycle**: Branches are created from `main`, developed, and merged back via Pull Requests after passing code review and CI.

## Release Branches
- **Naming Convention**: `release/<version>`.
- Used for final polishing and metadata updates before merging into `main` and tagging.
