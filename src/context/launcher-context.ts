import { createContextId } from "@builder.io/qwik";
import { AppViewModel, GothicGame } from "../types/launcher";

export interface AppStore {
  viewModel: AppViewModel | null;
  selectedGame: GothicGame;
}

export const LauncherContext = createContextId<AppStore>("launcher-context");
