export enum GothicGame {
  Gothic1 = "Gothic1",
  Gothic2 = "Gothic2",
  Gothic2NotR = "Gothic2NotR",
  ChroniclesOfMyrtana = "ChroniclesOfMyrtana",
}

export interface GameState {
  installPath: string | null;
  detected: boolean;
}

export interface ThemeConfig {
  bgColor: string;
  panelBg: string;
  sidebarBg: string;
  accentColor: string;
  textPrimary: string;
  textSecondary: string;
}

export interface LauncherConfig {
  activeEngine: string | null;
  activeProfile: string | null;
  games: Record<string, GameState>;
  theme: ThemeConfig;
}

export interface AppViewModel {
  config: LauncherConfig;
  installedEngines: EngineVersion[];
  availableReleases: EngineRelease[];
  backgroundTask: DownloadProgress | null;
}

export interface EngineVersion {
  version: string;
  executablePath: string;
}

export interface EngineRelease {
  tag: string;
  name: string;
  assets: EngineAsset[];
}

export interface EngineAsset {
  name: string;
  downloadUrl: string;
  size: number;
}

export interface DownloadProgress {
  received: number;
  total: number;
  percentage: number;
}
