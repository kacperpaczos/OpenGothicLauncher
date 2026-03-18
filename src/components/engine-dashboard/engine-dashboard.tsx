import { component$, $, useContext, type QRL } from "@builder.io/qwik";
import { invoke } from "@tauri-apps/api/core";
import { LauncherContext } from "../../context/launcher-context";
import { EngineRelease, LauncherConfig } from "../../types/launcher";

interface EngineDashboardProps {
  "close$": QRL<() => void>;
}

export const EngineDashboard = component$<EngineDashboardProps>((props) => {
  const launcherState = useContext(LauncherContext);
  
  const installedEngines = launcherState.viewModel?.installedEngines || [];
  const availableReleases = launcherState.viewModel?.availableReleases || [];
  const downloadProgress = launcherState.viewModel?.backgroundTask;
  const activeEngine = launcherState.viewModel?.config.activeEngine;

  const handleDownload = $(async (release: EngineRelease) => {
    try {
      await invoke("download_engine", { version: release.tag });
    } catch (e) {
      alert(`Download failed: ${e}`);
    }
  });

  const handleSetActive = $(async (version: string) => {
    try {
      const config = launcherState.viewModel?.config;
      if (!config) return;
      const updatedConfig: LauncherConfig = {
        ...config,
        activeEngine: version,
      };
      await invoke("save_config", { config: updatedConfig });
    } catch (e) {
      alert(`Failed to set active engine: ${e}`);
    }
  });

  return (
    <div class="modal-overlay" onClick$={props["close$"]}>
      <div class="modal-content engine-dashboard-modal" onClick$={(e) => e.stopPropagation()}>
        {/* Header */}
        <div class="dashboard-header">
          <div>
            <h2 style={{ margin: 0 }}>⚙ Engine Dashboard</h2>
            <p class="text-secondary" style={{ margin: "4px 0 0 0", fontSize: "0.85rem" }}>
              Manage OpenGothic engine versions
            </p>
          </div>
          <button class="btn btn-secondary btn-close-modal" onClick$={props["close$"]}>
            ✕
          </button>
        </div>

        {/* Download Progress */}
        {downloadProgress && (
           <div class="download-progress-bar">
              <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px", fontSize: "0.9rem" }}>
                 <span>⬇ Downloading...</span>
                 <span style={{ fontWeight: "bold", color: "var(--accent-color)" }}>{Math.round(downloadProgress.percentage)}%</span>
              </div>
              <div class="progress-track">
                 <div class="progress-fill" style={{ width: `${downloadProgress.percentage}%` }} />
              </div>
           </div>
        )}

        {/* Dashboard Grid */}
        <div class="dashboard-grid">
          {/* Installed Engines */}
          <div class="dashboard-section">
            <h3 class="section-title">
              <span class="section-icon">📦</span> Installed Engines
              <span class="badge">{installedEngines.length}</span>
            </h3>
            {installedEngines.length === 0 ? (
              <div class="empty-state">
                <span style={{ fontSize: "2rem", display: "block", marginBottom: "8px" }}>📭</span>
                <p>No engines installed yet.</p>
                <p class="text-secondary" style={{ fontSize: "0.85rem" }}>Download one from the available releases.</p>
              </div>
            ) : (
              <div class="engine-list">
                {installedEngines.map((engine) => {
                  const isActive = activeEngine === engine.version;
                  return (
                    <div key={engine.version} class={`engine-item ${isActive ? 'engine-item-active' : ''}`}>
                      <div style={{ flex: 1, minWidth: 0 }}>
                        <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                          <strong>{engine.version}</strong>
                          {isActive && <span class="active-badge">Active</span>}
                        </div>
                        <div class="text-secondary engine-path">
                          {engine.executablePath}
                        </div>
                      </div>
                      {!isActive && (
                        <button 
                          class="btn btn-primary" 
                          onClick$={() => handleSetActive(engine.version)}
                          style={{ padding: "6px 14px", fontSize: "0.85rem", whiteSpace: "nowrap" }}
                        >
                          Set Active
                        </button>
                      )}
                    </div>
                  );
                })}
              </div>
            )}
          </div>

          {/* Available Downloads */}
          <div class="dashboard-section">
            <h3 class="section-title">
              <span class="section-icon">☁</span> Available Downloads
              <span class="badge">{availableReleases.length}</span>
            </h3>
            {availableReleases.length === 0 ? (
              <div class="empty-state">
                <span style={{ fontSize: "2rem", display: "block", marginBottom: "8px" }}>🔄</span>
                <p>No releases available.</p>
                <p class="text-secondary" style={{ fontSize: "0.85rem" }}>Check your internet connection.</p>
              </div>
            ) : (
              <div class="engine-list" style={{ maxHeight: "400px", overflowY: "auto" }}>
                {availableReleases.map((release) => {
                  const isInstalled = installedEngines.some(e => e.version === release.tag);
                  const isDownloading = downloadProgress !== null && downloadProgress !== undefined;

                  return (
                    <div key={release.tag} class={`engine-item ${isInstalled ? 'engine-item-installed' : ''}`}>
                      <div style={{ flex: 1, minWidth: 0 }}>
                        <strong>{release.name || release.tag}</strong>
                        {release.name !== release.tag && (
                          <span class="text-secondary" style={{ marginLeft: "8px", fontSize: "0.85rem" }}>
                            ({release.tag})
                          </span>
                        )}
                      </div>
                      <button 
                        class={`btn ${isInstalled ? 'btn-secondary' : 'btn-primary'}`} 
                        onClick$={() => !isInstalled && handleDownload(release)}
                        disabled={isInstalled || isDownloading}
                        style={{ padding: "6px 14px", fontSize: "0.85rem", whiteSpace: "nowrap" }}
                      >
                        {isInstalled ? "✓ Installed" : "⬇ Download"}
                      </button>
                    </div>
                  )
                })}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
});
