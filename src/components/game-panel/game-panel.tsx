import { component$, $, useContext, useSignal } from "@builder.io/qwik";
import { GothicGame, GameState } from "../../types/launcher";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { LauncherContext } from "../../context/launcher-context";
import { EngineDashboard } from "../engine-dashboard/engine-dashboard";

interface GamePanelProps {
  game: GothicGame;
  state: GameState;
}

export const GamePanel = component$<GamePanelProps>(({ game, state }) => {
  const launcherState = useContext(LauncherContext);
  const isDashboardOpen = useSignal(false);

  const handleScan = $(async () => {
    try {
      await invoke("scan_for_games");
    } catch (e) {
      console.error("Scan failed", e);
    }
  });

  const handleManualSelect = $(async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Gothic Installation Folder",
      });

      if (selected && typeof selected === "string") {
        await invoke("manual_select_game_path", { game, path: selected });
      }
    } catch (e) {
      console.error("Manual selection failed", e);
    }
  });

  const handleLaunch = $(async () => {
    try {
      await invoke("launch_game", { game });
    } catch (e) {
      alert(`Launch failed: ${e}`);
    }
  });

  const hasEngine = launcherState.viewModel?.config.activeEngine !== null;
  const downloadProgress = launcherState.viewModel?.backgroundTask;
  const metadata = launcherState.viewModel?.libraryMetadata[game];

  if (!metadata) return null;

  return (
    <div class="center-panel">
      {/* Cinematic Banner */}
      <div class="game-banner" style={{ backgroundImage: `url('${metadata.bannerUrl}')` }}>
        <div class="banner-overlay">
          <div class="banner-text">
            {metadata.subtitle && <div class="banner-subtitle">{metadata.subtitle}:</div>}
            <h1 class="banner-title">{metadata.title}</h1>
          </div>
        </div>
      </div>

      {/* Game Description */}
      <div class="game-description">
        <div class="game-description-card">
          <p class="game-description-text">{metadata.description}</p>
        </div>
      </div>

      {/* Action Buttons */}
      <div class="action-buttons">
        {(!state.detected || !state.installPath) ? (
          <>
            {!state.installPath && state.detected && (
              <p class="status-error" style={{ width: '100%', textAlign: 'center', marginBottom: '1rem', color: 'var(--accent-color)' }}>
                ⚠ Installation path missing. Please select the game folder again.
              </p>
            )}
            <button class="btn btn-primary btn-lg" onClick$={handleScan}>
              🔍 Scan for game
            </button>
            <button class="btn btn-secondary btn-lg" onClick$={handleManualSelect}>
              📁 Select folder
            </button>
          </>
        ) : (
          <>
            {hasEngine ? (
              <button class="btn btn-primary btn-lg" onClick$={handleLaunch}>
                ▶ Launch Game
              </button>
            ) : (
              <button 
                class="btn btn-primary btn-lg" 
                onClick$={() => { isDashboardOpen.value = true; }}
              >
                ⚙ Install Engine
              </button>
            )}
            <button
              class="btn btn-secondary btn-lg"
              onClick$={() => {
                isDashboardOpen.value = true;
              }}
            >
              ⚙ Manage Engines
            </button>
          </>
        )}
      </div>

      {/* Download Progress */}
      {downloadProgress && (
        <div class="download-progress-bar">
          <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px", fontSize: "0.9rem" }}>
            <span>⬇ Downloading engine...</span>
            <span style={{ fontWeight: "bold", color: "var(--accent-color)" }}>
              {Math.round(downloadProgress.percentage)}%
            </span>
          </div>
          <div class="progress-track">
            <div class="progress-fill" style={{ width: `${downloadProgress.percentage}%` }} />
          </div>
        </div>
      )}

      {/* Engine Dashboard Modal */}
      {isDashboardOpen.value && (
        <EngineDashboard close$={$(() => { isDashboardOpen.value = false; })} />
      )}
    </div>
  );
});
