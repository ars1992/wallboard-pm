import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { register, isRegistered, unregister } from "@tauri-apps/plugin-global-shortcut";
import { invoke } from "@tauri-apps/api/core";

(async () => {
  const shortcut = "CommandOrControl+Shift+W";

  // dev/hotreload-sicher: falls schon registriert, erst entfernen
  try {
    if (await isRegistered(shortcut)) {
      await unregister(shortcut);
    }

    await register(shortcut, (event) => {
      if (event.state === "Pressed") {
        invoke("open_settings");
      }
    });
  } catch (e) {
    console.warn("Global shortcut registration failed:", e);
  }
})();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);