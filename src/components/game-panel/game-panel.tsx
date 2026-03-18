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

const gameDisplayNames: Record<GothicGame, { title: string; subtitle?: string }> = {
  [GothicGame.Gothic1]: { title: "Gothic" },
  [GothicGame.Gothic2]: { title: "Gothic II" },
  [GothicGame.Gothic2NotR]: { title: "Gothic II", subtitle: "Night of the Raven" },
  [GothicGame.ChroniclesOfMyrtana]: { title: "Archolos", subtitle: "The Chronicles of Myrtana" },
};

const gameBanners: Record<GothicGame, string> = {
  [GothicGame.Gothic1]: "/banner-archolos.png",
  [GothicGame.Gothic2]: "/banner-archolos.png",
  [GothicGame.Gothic2NotR]: "/banner-archolos.png",
  [GothicGame.ChroniclesOfMyrtana]: "/banner-archolos.png",
};

const gameDescriptions: Record<GothicGame, string> = {
  [GothicGame.Gothic1]:
    "Gothic to kultowe RPG akcji z 2001 roku, osadzone w mrocznym świecie fantasy. Wcielasz się w Bezimiennego, skazańca wrzuconego do Kolonii Karnej — gigantycznego więzienia otoczonego magiczną barierą. Eksploruj obóz górniczy, walcz z potworami i odkrywaj sekrety pradawnej magii.",
  [GothicGame.Gothic2]:
    "Gothic II kontynuuje przygodę Bezimiennego po upadku Bariery. Miasto Khorinis i otaczające je tereny są zagrożone przez armię ciemności. Dołącz do Paladynów, Magów Ognia lub Najemników i powstrzymaj nadciągające zło w jednym z najlepszych RPG w historii.",
  [GothicGame.Gothic2NotR]:
    "Night of the Raven — dodatek do Gothic II, który rozszerza świat gry o Jharkendar, starożytną krainę ukrytą w głębi wyspy. Nowe potwory, przedmioty, questline'y i zwiększony poziom trudności czynią tę wersję definitywnym doświadczeniem Gothic II.",
  [GothicGame.ChroniclesOfMyrtana]:
    "The Chronicles of Myrtana: Archolos to pełnoprawna gra RPG stworzona na silniku Gothic II. Rozgrywa się na wyspie Archolos i oferuje ponad 100 godzin rozgrywki z nową fabułą, postaciami, muzyką i ogromnym, ręcznie tworzonym światem. Uznawana za jedną z najlepszych modyfikacji w historii gier.",
};

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
  const display = gameDisplayNames[game];

  return (
    <div class="center-panel">
      {/* Cinematic Banner */}
      <div class="game-banner" style={{ backgroundImage: `url('${gameBanners[game]}')` }}>
        <div class="banner-overlay">
          <div class="banner-text">
            {display.subtitle && <div class="banner-subtitle">{display.subtitle}:</div>}
            <h1 class="banner-title">{display.title}</h1>
          </div>
        </div>
      </div>

      {/* Game Description */}
      <div class="game-description">
        <div class="game-description-card">
          <p class="game-description-text">{gameDescriptions[game]}</p>
        </div>
      </div>

      {/* Action Buttons */}
      <div class="action-buttons">
        {!state.detected ? (
          <>
            <button class="btn btn-primary btn-lg" onClick$={handleScan}>
              🔍 Scan for game
            </button>
            <button class="btn btn-secondary btn-lg" onClick$={handleManualSelect}>
              📁 Select folder
            </button>
          </>
        ) : (
          <>
            {hasEngine && (
              <button class="btn btn-primary btn-lg" onClick$={handleLaunch}>
                ▶ Launch Game
              </button>
            )}
            <button
              class={`btn ${hasEngine ? "btn-secondary" : "btn-primary"} btn-lg`}
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

      {/* Recent Activity */}
      <div class="recent-activity">
        <h3 class="section-heading">Recent Activity</h3>
        <div class="activity-list">
          <div class="activity-item">
            <div class="activity-text">Application started. Checking for engine updates...</div>
            <div class="activity-date">{new Date().toLocaleDateString("pl-PL", { month: "long", day: "numeric", year: "numeric" })}</div>
          </div>
          {state.detected && (
            <div class="activity-item">
              <div class="activity-text">
                Game installation detected at <code>{state.installPath}</code>
              </div>
              <div class="activity-date">{new Date().toLocaleDateString("pl-PL", { month: "long", day: "numeric", year: "numeric" })}</div>
            </div>
          )}
          {launcherState.viewModel?.config.activeEngine && (
            <div class="activity-item">
              <div class="activity-text">
                Engine version <strong>{launcherState.viewModel.config.activeEngine}</strong> set as active.
              </div>
              <div class="activity-date">{new Date().toLocaleDateString("pl-PL", { month: "long", day: "numeric", year: "numeric" })}</div>
            </div>
          )}
        </div>
      </div>

      {/* Engine Dashboard Modal */}
      {isDashboardOpen.value && (
        <EngineDashboard close$={$(() => { isDashboardOpen.value = false; })} />
      )}
    </div>
  );
});
