# Plugin or Extension System

Currently, the OpenGothicLauncher does not feature a dedicated plugin system for third-party developers. However, the internal architecture is designed with "extension points" in mind.

## Extension Points
1.  **Ports and Adapters**: A developer can add support for a new platform or networking backend by implementing the relevant `ports` in a new crate and injecting it into the `LauncherService`.
2.  **Custom Runners**: The `GameProcessRunner` trait can be implemented to support alternative execution methods (e.g., running via Steam's `steam-runtime-launch-options`).
3.  **Game Support**: Supporting new Gothic variants or mod-total-conversions requires adding new variants to the `GothicGame` enum and updating the detection rules in `ogl-infra`.

## Planned Plugin Features
- **Scriptable Mod Logic**: Potential integration of a scripting language (e.g., Rhai or Lua) to allow users to define custom load orders or pre-launch scripts.
- **UI Themes**: CSS-based theme injection for the GTK interface.
