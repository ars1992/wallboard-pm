// src/settings/settings.ts
import "./settings.css";

import { invoke } from "@tauri-apps/api/core";
import type { AppConfig, MonitorInfo } from "./types";

const $ = (id: string) => document.getElementById(id)!;

async function loadConfig(): Promise<AppConfig> {
  return await invoke<AppConfig>("get_config");
}

async function loadMonitors(): Promise<MonitorInfo[]> {
  return await invoke<MonitorInfo[]>("list_monitors");
}

function setStatus(msg: string, type: "ok" | "err" | "muted" = "muted") {
  const el = $("status");
  el.textContent = msg;
  el.className = "status " + (type === "ok" ? "statusOk" : type === "err" ? "statusErr" : "");
}

function renderViews(cfg: AppConfig) {
  const host = $("views");
  host.innerHTML = "";

  cfg.views.forEach((v, idx) => {
    const card = document.createElement("div");
    card.className = "viewCard";

    card.innerHTML = `
      <div class="viewTop">
        <div class="viewName">View ${idx + 1}</div>
        <div class="viewId">${v.id}</div>
      </div>

      <div class="viewGrid">
        <div>
          <div class="label">URL</div>
          <input class="input" data-view="${idx}" value="${escapeHtml(v.url)}" placeholder="https://..." />
        </div>

        <div>
          <div class="label">Profil</div>
          <input class="input" data-profile="${idx}" value="${escapeHtml(v.profile ?? "")}" placeholder="view${idx + 1}" />
        </div>
      </div>
    `;

    host.appendChild(card);
  });
}

// simple escaping because we inject into innerHTML
function escapeHtml(s: string) {
  return s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

function readConfigFromUI(cfg: AppConfig): AppConfig {
  const mode = ($("monitorMode") as HTMLSelectElement).value as AppConfig["monitor"]["mode"];
  const value = ($("monitorValue") as HTMLInputElement).value.trim();

  const urls = Array.from(document.querySelectorAll<HTMLInputElement>("input[data-view]"));
  const profiles = Array.from(document.querySelectorAll<HTMLInputElement>("input[data-profile]"));

  const views = cfg.views.map((v, i) => ({
    ...v,
    url: (urls[i]?.value ?? "").trim(),
    profile: (profiles[i]?.value ?? "").trim() || null,
  })) as AppConfig["views"];

  return {
    ...cfg,
    monitor: { mode, value: value || null },
    views,
  };
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

  $("refreshMonitors").addEventListener("click", () => {
    refreshMonitors().catch(err => setStatus(String(err), "err"));
  });

  await refreshMonitors();

  $("save").addEventListener("click", async () => {
    try {
      cfg = readConfigFromUI(cfg);
      await invoke("save_config", { newCfg: cfg });
      setStatus("Gespeichert.", "ok");
    } catch (e: any) {
      setStatus(e?.message ?? String(e), "err");
    }
  });

  $("saveApply").addEventListener("click", async () => {
    try {
      cfg = readConfigFromUI(cfg);
      await invoke("save_config", { newCfg: cfg });
      await invoke("apply_config");
      setStatus("Gespeichert & angewendet.", "ok");
    } catch (e: any) {
      setStatus(e?.message ?? String(e), "err");
    }
  });
})();