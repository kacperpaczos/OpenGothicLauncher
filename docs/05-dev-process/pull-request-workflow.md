# Pull Request Workflow

## Workflow Steps
1.  **Drafting**: Create a PR as a "Draft" early in the process to signal ongoing work.
2.  **Self-Review**: Review your own diffs for obvious issues or missing documentation.
3.  **Review Request**: Tag relevant maintainers for review.
4.  **Iteration**: Address review comments, pushing new commits to the same branch.
5.  **Merge**: Once approved and CI passes, use "Squash and Merge" into `main` to maintain a clean git history.

## PR Requirements
- All new features must be accompanied by relevant documentation in `docs/`.
- Must pass `cargo test` and `cargo clippy`.
- Commit messages should follow conventional commits (e.g., `feat: added engine manager`, `fix: path detection on Linux`).
