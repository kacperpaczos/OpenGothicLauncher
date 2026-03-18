import { component$, Slot, useContext } from "@builder.io/qwik";
import { LauncherContext } from "../../context/launcher-context";

export const ThemeProvider = component$(() => {
  const store = useContext(LauncherContext);
  const theme = store.viewModel?.config.theme;

  if (!theme) {
    return <Slot />;
  }

  return (
    <div
      style={{
        "--bg-color": theme.bgColor,
        "--panel-bg": theme.panelBg,
        "--sidebar-bg": theme.sidebarBg,
        "--accent-color": theme.accentColor,
        "--text-primary": theme.textPrimary,
        "--text-secondary": theme.textSecondary,
        // Map these straight to Tailwind / Qwik UI internals
        "--background": theme.bgColor,
        "--foreground": theme.textPrimary,
        "--primary": theme.accentColor,
        "--primary-foreground": theme.bgColor,
        "--muted": theme.panelBg,
        "--muted-foreground": theme.textSecondary,
        "--border": theme.sidebarBg,
      }}
      class="min-h-screen text-$text-primary bg-$bg-color"
    >
      <Slot />
    </div>
  );
});
