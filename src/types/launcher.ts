export enum GothicGame {
  Gothic1 = "Gothic1",
  Gothic2 = "Gothic2",
  Gothic2NotR = "Gothic2NotR",
  ChroniclesOfMyrtana = "ChroniclesOfMyrtana",
  Gothic3 = "Gothic3",
}

export interface GameState {
  installPath: string | null;
  detected: boolean;
}

export interface GameMetadata {
  title: string;
  subtitle: string | null;
  description: string;
  bannerUrl: string;
}

export interface ModboxConfig {
  mods: string[];
}

export interface Profile {
  id: string;
  name: string;
  game: GothicGame;
  engineVersion: string | null;
  modbox: ModboxConfig;
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
  activeProfileId: string | null;
  games: Record<string, GameState>;
  profiles: Profile[];
  theme: ThemeConfig;
}

export interface AppViewModel {
  config: LauncherConfig;
  installedEngines: EngineVersion[];
  availableReleases: EngineRelease[];
  backgroundTask: DownloadProgress | null;
  libraryMetadata: Record<string, GameMetadata>;
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
