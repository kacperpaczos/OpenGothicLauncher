# Design Patterns

The codebase employs several classic and modern design patterns to achieve clean separation and robustness.

## 1. Adapter Pattern (Hexagonal Architecture)
The primary pattern. `ogl-infra` and `ogl-network` act as adapters providing concrete implementations for the `ports` defined in `ogl-core`.

## 2. Dependency Injection (DI)
Dependencies are injected into services during the bootstrap phase (in `main.rs`). We use "Constructor Injection" by passing `Arc<dyn Port>` to service constructors.

## 3. MVVM (Model-View-ViewModel)
Implemented in `ogl-gui`:
- **Model**: `ogl-core` domain objects.
- **View**: GTK4 widgets in `src/*.rs`.
- **ViewModel**: Structs in `src/view_models/*.rs` that encapsulate UI state and handle commands from the View.

## 4. Strategy Pattern
Used in `InstallDetector`. The system can switch between different detection strategies (Fast, Heuristic, Brute-Force) based on user interaction or failure states.

## 5. Result/Option Combinators
Extensive use of functional patterns for error handling and data transformation, leveraging Rust's `Result` and `Option` types to avoid null pointers and unhandled exceptions.
