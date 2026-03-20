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

  const handleInstall = $(async (version: string) => {
    try {
      if (launcherState.viewModel) {
        launcherState.viewModel.backgroundTask = { received: 0, total: 100, percentage: 0 };
      }
      invoke("log_action", { action: "Install Engine", details: version }).catch(console.error);
      await invoke("download_engine", { version });
    } catch (e) {
      alert(`Install failed: ${e}`);
    } finally {
      await invoke("get_state");
    }
  });

  const handleDownload = $(async (release: EngineRelease) => {
    await handleInstall(release.tag);
  });

  const handleSetActive = $(async (version: string) => {
    try {
      invoke("log_action", { action: "Set Active Engine", details: version }).catch(console.error);
      const config = launcherState.viewModel?.config;
      if (!config) return;
      const updatedConfig: LauncherConfig = {
        ...config,
        activeEngine: version,
      };
      await invoke("save_config", { config: updatedConfig });
      await invoke("get_state");
    } catch (e) {
      alert(`Failed to set active engine: ${e}`);
    }
  });

  const handleDelete = $(async (version: string) => {
    if (confirm(`Are you sure you want to delete engine version ${version}?`)) {
      try {
        invoke("log_action", { action: "Delete Engine", details: version }).catch(console.error);
        await invoke("delete_engine", { version });
        await invoke("get_state");
      } catch (e) {
        alert(`Failed to delete engine: ${e}`);
      }
    }
  });

  const handleReinstall = $(async (version: string) => {
    try {
      invoke("log_action", { action: "Reinstall Engine", details: version }).catch(console.error);
      await invoke("reinstall_engine", { version });
      await invoke("get_state");
    } catch (e) {
      alert(`Failed to reinstall engine: ${e}`);
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
                 <span>⬇ Installing...</span>
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
                    <div 
                      key={engine.version} 
                      class={`engine-item ${isActive ? 'engine-item-active' : ''}`}
                      onClick$={() => !isActive && handleSetActive(engine.version)}
                      style={{ cursor: isActive ? 'default' : 'pointer' }}
                    >
                      <div style={{ flex: 1, minWidth: 0 }}>
                        <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                          <strong>{engine.version}</strong>
                          {isActive && <span class="active-badge">Active</span>}
                        </div>
                        <div class="text-secondary engine-path">
                          {engine.executablePath}
                        </div>
                      </div>
                      <div class="engine-actions">
                        {!isActive && (
                          <button 
                            class="btn btn-primary btn-sm" 
                            onClick$={(e) => {
                              e.stopPropagation();
                              handleSetActive(engine.version);
                            }}
                          >
                            Set Active
                          </button>
                        )}
                        <button 
                          class="btn btn-secondary btn-sm"
                          title="Reinstall"
                          onClick$={(e) => {
                            e.stopPropagation();
                            handleReinstall(engine.version);
                          }}
                        >
                          🔄
                        </button>
                        <button 
                          class="btn btn-secondary btn-sm btn-delete"
                          title="Delete"
                          onClick$={(e) => {
                            e.stopPropagation();
                            handleDelete(engine.version);
                          }}
                        >
                          🗑
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </div>

          {/* Available Downloads */}
          <div class="dashboard-section">
            <h3 class="section-title">
              <span class="section-icon">☁</span> Available Releases
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
                        {isInstalled ? "✓ Installed" : "⬇ Install"}
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
