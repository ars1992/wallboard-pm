// src/settings/settings.ts
import { invoke } from "@tauri-apps/api/core";
import type { AppConfig, MonitorInfo } from "./types";

const $ = (id: string) => document.getElementById(id)!;

async function loadConfig(): Promise<AppConfig> {
  return await invoke<AppConfig>("get_config");
}

async function loadMonitors(): Promise<MonitorInfo[]> {
  return await invoke<MonitorInfo[]>("list_monitors");
}

function renderViews(cfg: AppConfig) {
  const host = $("views");
  host.innerHTML = "";

  cfg.views.forEach((v, idx) => {
    const row = document.createElement("div");
    row.innerHTML = `
      <label>View ${idx + 1} (${v.id}) URL:
        <input data-view="${idx}" value="${v.url}" style="width: 520px" />
      </label>
      <label>Profil:
        <input data-profile="${idx}" value="${v.profile ?? ""}" placeholder="z.B. view1" />
      </label>
    `;
    host.appendChild(row);
  });
}

function readConfigFromUI(cfg: AppConfig): AppConfig {
  const mode = ($("monitorMode") as HTMLSelectElement).value as AppConfig["monitor"]["mode"];
  const value = ($("monitorValue") as HTMLInputElement).value.trim();

  const urls = Array.from(document.querySelectorAll<HTMLInputElement>("input[data-view]"));
  const profiles = Array.from(document.querySelectorAll<HTMLInputElement>("input[data-profile]"));

  const views = cfg.views.map((v, i) => ({
    ...v,
    url: urls[i].value.trim(),
    profile: profiles[i].value.trim() || null,
  })) as AppConfig["views"];

  return {
    ...cfg,
    monitor: { mode, value: value || null },
    views,
  };
}

function setStatus(msg: string) {
  $("status").textContent = msg;
}

(async () => {
  let cfg = await loadConfig();

  ($("monitorMode") as HTMLSelectElement).value = cfg.monitor.mode;
  ($("monitorValue") as HTMLInputElement).value = cfg.monitor.value ?? "";
  renderViews(cfg);

  async function refreshMonitors() {
    const list = await loadMonitors();
    $("monitors").textContent = list
      .map(m => `${m.index}: ${m.name} ${m.is_primary ? "(primary)" : ""} pos=${m.position} size=${m.size}`)
      .join("\n");
  }

  $("refreshMonitors").addEventListener("click", refreshMonitors);
  await refreshMonitors();

  $("save").addEventListener("click", async () => {
    cfg = readConfigFromUI(cfg);
    await invoke("save_config", { newCfg: cfg });
    setStatus("Gespeichert.");
  });

  $("saveApply").addEventListener("click", async () => {
    cfg = readConfigFromUI(cfg);
    await invoke("save_config", { newCfg: cfg });
    await invoke("apply_config");
    setStatus("Gespeichert & angewendet.");
  });
})();