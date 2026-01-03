import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { register, isRegistered, unregister } from "@tauri-apps/plugin-global-shortcut";
import { invoke } from "@tauri-apps/api/core";

const shortcut = "CommandOrControl+Shift+W";

await register(shortcut, (event) => {
  if (event.state === "Pressed") {
    invoke("open_settings").catch(console.error);
  }
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);