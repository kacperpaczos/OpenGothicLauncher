import { component$, useContext } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { LauncherContext } from "../context/launcher-context";
import { GamePanel } from "../components/game-panel/game-panel";
import { GameDetailsPanel } from "../components/game-details-panel/game-details-panel";

export default component$(() => {
  const state = useContext(LauncherContext);

  if (!state.viewModel) return null;

  const gameState = state.viewModel.config.games[state.selectedGame] || {
    installPath: null,
    detected: false,
  };

  return (
    <>
      <main class="main-content">
        <GamePanel game={state.selectedGame} state={gameState} />
      </main>
      <GameDetailsPanel game={state.selectedGame} />
    </>
  );
});

export const head: DocumentHead = {
  title: "OpenGothic Launcher",
  meta: [
    {
      name: "description",
      content: "Modern launcher for Gothic games",
    },
  ],
};
