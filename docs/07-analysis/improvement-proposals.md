# Improvement Proposals

## 1. Actor-Based State Management
Move away from a shared Mutex toward an Actor-based message-passing system.
- **Benefit**: Removes lock contention and makes state transitions more explicit and observable.
- **Implementation**: Could use `tokio` channels or a specialized actor library like `actix`.

## 2. Integrated Plugin System
Implement a formal interface for community-contributed "Translators" and "Launchers".
- **Benefit**: Allows the project to support every possible Gothic total conversion without bloating the core.

## 3. Remote Configuration Injection
Allow OpenGothic to be configured directly via the launcher without modifying `.ini` files.
- **Benefit**: Safer and more user-friendly configuration.
- **Implementation**: Feed configuration parameters as CLI arguments to OpenGothic.

## 4. Native Linux Packaging
Provide natvie `.deb`, `.rpm`, or AppImage packages.
- **Benefit**: Easier installation for Linux users who don't want to compile from source.
