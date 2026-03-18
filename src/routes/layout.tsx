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
  });

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

    return () => {
       unlistenState();
       unlistenProgress();
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
