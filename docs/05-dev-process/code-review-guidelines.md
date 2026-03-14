# Code Review Guidelines

## Reviewer Focus
- **Architectural Alignment**: Does the change respect the Clean Architecture boundaries?
- **Error Handling**: Are new failure points handled with appropriate `Result` types and domain errors?
- **Maintainability**: Is the code easy to read? Does it avoid "magic numbers" and over-complicated logic?
- **Ownership**: Does the code respect Rust's safety rules? Are `Arc` and `Mutex` used correctly?

## Communication
- Provide constructive feedback.
- Use "Nitpick" for minor styling issues that shouldn't block the merge.
- Explain the *why* behind requested changes to help team learning.
