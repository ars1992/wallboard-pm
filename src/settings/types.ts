// src/settings/types.ts
export type MonitorMode = "primary" | "index" | "name_contains";

export interface MonitorSelector {
  mode: MonitorMode;
  value?: string | null;
}

export interface ViewConfig {
  id: string;
  url: string;
  profile?: string | null;
}

export interface AppConfig {
  version: number;
  monitor: MonitorSelector;
  views: [ViewConfig, ViewConfig, ViewConfig, ViewConfig];
}

export interface MonitorInfo {
  index: number;
  name: string;
  is_primary: boolean;
  position: [number, number];
  size: [number, number];
}