import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { register } from "@tauri-apps/plugin-global-shortcut";
import { invoke } from "@tauri-apps/api/core";

const shortcutSettings = "CommandOrControl+Shift+W";

const shortcutMin = "CommandOrControl+Shift+S";

await register(shortcutMin, (event) => {
  if (event.state === "Pressed") {
    invoke("toggle_minimize_views").catch(console.error);
  }
});

await register(shortcutSettings, (event) => {
  if (event.state === "Pressed") {
    invoke("open_settings").catch(console.error);
  }
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);