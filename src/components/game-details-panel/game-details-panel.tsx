import { component$, useContext } from "@builder.io/qwik";
import { LauncherContext } from "../../context/launcher-context";
import { GothicGame } from "../../types/launcher";

interface GameDetailsPanelProps {
  game: GothicGame;
}

export const GameDetailsPanel = component$<GameDetailsPanelProps>(({ game }) => {
  const launcherState = useContext(LauncherContext);

  const gameState = launcherState.viewModel?.config.games[game];
  const activeEngine = launcherState.viewModel?.config.activeEngine;
  const installedEngines = launcherState.viewModel?.installedEngines || [];

  const currentEngine = activeEngine
    ? installedEngines.find((e) => e.version === activeEngine)
    : null;

  return (
    <aside class="details-panel">
      {/* Game Status Card */}
      <div class="detail-card">
        <h3 class="detail-card-title">
          <span class="detail-icon">🎮</span> Game Status
        </h3>
        <div class="detail-card-body">
          <div class="detail-row">
            <span class="detail-label">⏱ Total Playtime:</span>
            <span class="detail-value">—</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">🕐 Last Played:</span>
            <span class="detail-value">—</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">💾 Last Save State:</span>
            <span class="detail-value detail-value-wrap">—</span>
          </div>
          <div class="detail-screenshot">
            <div class="screenshot-placeholder">
              <span>No screenshot</span>
            </div>
          </div>
        </div>
      </div>

      {/* Engine & Mods Card */}
      <div class="detail-card">
        <h3 class="detail-card-title">
          <span class="detail-icon">⚙</span> Engine & Mods
        </h3>
        <div class="detail-card-body">
          <div class="detail-row">
            <span class="detail-label">Current Engine:</span>
            <span class="detail-value">
              {currentEngine ? (
                <span class="engine-active-label">
                  <span class="status-dot status-dot-ok"></span>
                  OpenGothic ({currentEngine.version})
                </span>
              ) : (
                <span class="engine-none-label">Not set</span>
              )}
            </span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Active Patches:</span>
            <span class="detail-value">—</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Active Textures:</span>
            <span class="detail-value">—</span>
          </div>
        </div>
      </div>

      {/* Status Bar */}
      <div class="status-bar">
        {gameState?.detected ? (
          <div class="status-bar-item">
            <span class="status-dot status-dot-ok"></span>
            <span>Found at: <code class="path-label">{gameState.installPath}</code></span>
          </div>
        ) : (
          <div class="status-bar-item">
            <span class="status-dot status-dot-err"></span>
            <span>Not detected</span>
          </div>
        )}
        <div class="status-bar-tray">
          <span class="tray-icon">🔊</span>
          <span class="tray-icon">📶</span>
          <span class="tray-icon">🔋</span>
          <div class="status-bar-time">
            {new Date().toLocaleTimeString("pl-PL", { hour: "2-digit", minute: "2-digit" })}
            <span class="text-secondary" style={{ marginLeft: "6px", fontSize: "0.75rem" }}>
              {new Date().toLocaleDateString("pl-PL")}
            </span>
          </div>
        </div>
      </div>
    </aside>
  );
});
