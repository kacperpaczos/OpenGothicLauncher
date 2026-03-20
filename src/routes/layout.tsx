import { component$, Slot, useStore, $, useVisibleTask$, useContextProvider } from "@builder.io/qwik";
import { Sidebar } from "../components/sidebar/sidebar";
import { GothicGame, AppViewModel, DownloadProgress } from "../types/launcher";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { LauncherContext, AppStore } from "../context/launcher-context";
import { ThemeProvider } from "../components/theme-provider/theme-provider";

export default component$(() => {
  const state = useStore<AppStore>({
    selectedGame: GothicGame.Gothic1,
    viewModel: null,
  }, { deep: true });

  useContextProvider(LauncherContext, state);

  const requestState = $(async () => {
    await invoke("get_state");
  });

  const handleGameSelect = $((game: GothicGame) => {
    state.selectedGame = game;
  });

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    const unlistenState = await listen<AppViewModel>("state_updated", (event) => {
       state.viewModel = event.payload;
    });

    const unlistenProgress = await listen<DownloadProgress>("download-progress", (event) => {
      if (state.viewModel) {
        if (event.payload.percentage >= 100) {
          // Download complete — clear the progress indicator
          state.viewModel.backgroundTask = null;
        } else {
          state.viewModel.backgroundTask = event.payload;
        }
      }
    });

    await requestState();

    // Global click logger
    const clickHandler = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (!target) return;
      
      let details = "";
      if (target.id) details += `id: ${target.id}`;
      if (target.className) details += `${details ? ", " : ""}class: ${target.className}`;
      if (target.innerText) details += `${details ? ", " : ""}text: ${target.innerText.substring(0, 30)}`;

      invoke("log_action", { 
        action: `Click on <${target.tagName.toLowerCase()}>`, 
        details 
      }).catch(console.error);
    };

    window.addEventListener("click", clickHandler);

    return () => {
       unlistenState();
       unlistenProgress();
       window.removeEventListener("click", clickHandler);
    };
  });

  return (
    <ThemeProvider>
      <div class="app-container">
        <Sidebar 
          selectedGame={state.selectedGame} 
          onGameSelect$={handleGameSelect} 
        />
        {state.viewModel ? (
           <Slot />
        ) : (
          <div class="spinner-container">
            <div class="spinner"></div>
            <div class="loading-text">Ładowanie konfiguracji...</div>
          </div>
        )}
      </div>
    </ThemeProvider>
  );
});
