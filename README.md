````markdown
# wallboard-pm

Tauri-basierte Desktop-App, die **vier Webseiten gleichzeitig** in einem **2×2 Grid** anzeigt – umgesetzt als **vier borderlose WebView-Fenster** (statt iframes).

Ziel: **stabiler Dauerbetrieb auf Windows 11** (z. B. Samsung *The Frame*).  
Entwicklung auf **macOS**, Windows-Builds via **GitHub Actions**.

---

## Inhalt

- [Features](#features)
- [Screens / UX](#screens--ux)
- [Installation (Windows 11)](#installation-windows-11)
- [Entwicklung (macOS)](#entwicklung-macos)
- [Konfiguration](#konfiguration)
- [Shortcuts](#shortcuts)
- [Build & Release (MSI via Tags)](#build--release-msi-via-tags)
- [Troubleshooting](#troubleshooting)
- [Projektstruktur](#projektstruktur)
- [Issues: Bugs & Feature Requests](#issues-bugs--feature-requests)
- [Security](#security)
- [Roadmap (kurz)](#roadmap-kurz)
- [Lizenz](#lizenz)

---

## Features

- **4 Webseiten gleichzeitig** (2×2 Grid)
- Jede Webseite läuft in einem eigenen **Tauri WebView Window**
  - unabhängig bedienbar (Scroll, Klick, Fokus)
  - keine iframe-Einschränkungen durch `X-Frame-Options` / `frame-ancestors`
- **Runtime-Konfiguration** via `config.json` im App-Config-Verzeichnis (ohne Neu-Build)
  - URLs ändern
  - Ziel-Monitor wählen (Primary / Index / Name contains)
- **Persistente Sessions pro View** über `profile`
  - Cookies & Storage bleiben erhalten
  - Ziel: *einmal einloggen, Session bleibt*
- **Settings-Fenster** per Hotkey (inkl. *Advanced Toggle*)
- **Toggle: Views minimieren / wiederherstellen** per Hotkey
- **Windows MSI Release** via GitHub Actions bei Tags (`vX.Y.Z`)

---

## Screens / UX

- **View-Fenster**
  - vier borderlose Fenster
  - gekachelt im 2×2 Grid
  - optional `always_on_top` (Kiosk-Style)
- **Settings-Fenster**
  - separate UI zum Bearbeiten der URLs & Monitor-Auswahl
  - *Advanced Toggle* blendet Profil-Einstellungen ein/aus

---

## Installation (Windows 11)

### MSI installieren

1. MSI aus dem jeweiligen **GitHub Release** (Assets) herunterladen
2. Installer (`.msi`) ausführen
3. App starten

### Erster Start

- Beim ersten Start wird automatisch eine `config.json` im **App-Config-Verzeichnis** erzeugt
- Öffne die Settings mit `Cmd/Ctrl + Shift + S`
- URLs eintragen → **Speichern & Anwenden**

> **Hinweis:**  
> Windows 11 bringt WebView2 in der Regel mit. Falls nicht, muss die **WebView2 Runtime** nachinstalliert werden.

---

## Entwicklung (macOS)

### Voraussetzungen

- Node.js + npm
- Rust (stable) + Cargo
- Xcode Command Line Tools

Check:

```bash
node -v
npm -v
rustc -V
cargo -V
````

### Projekt starten

```bash
npm install
npm run tauri dev
```

**Hinweise**

* macOS nutzt **WKWebView**
* Windows nutzt **WebView2**
* Verhalten kann leicht abweichen
* Windows MSI wird ausschließlich über CI gebaut

---

## Konfiguration

### `config.json`

Die App nutzt eine Konfiguration im App-Config-Verzeichnis.

Beispiel:

```json
{
  "version": 1,
  "monitor": { "mode": "name_contains", "value": "Samsung" },
  "views": [
    { "id": "topLeft", "url": "https://example.com", "profile": "view1" },
    { "id": "topRight", "url": "https://example.org", "profile": "view2" },
    { "id": "bottomLeft", "url": "https://example.net", "profile": "view3" },
    { "id": "bottomRight", "url": "https://www.wikipedia.org", "profile": "view4" }
  ]
}
```

### Monitor-Auswahl

`monitor.mode`:

* `primary` – Primärmonitor
* `index` – Monitor per Index (0, 1, 2, …)
* `name_contains` – Monitorname enthält Substring (z. B. `"Samsung"`)

### Profile (Sessions)

`profile` definiert das persistente WebView-Profil pro View:

* Cookies
* LocalStorage
* Cache

**Vorteile**

* Login bleibt erhalten
* Unterschiedliche Accounts parallel möglich

**Wichtig**

* Profilwechsel sind nicht immer „hot“ möglich
* URLs & Monitor lassen sich live anwenden
* Profile gelten als **Advanced Feature**

---

## Shortcuts

Alle Shortcuts sind plattformübergreifend (`CommandOrControl`):

* **Settings öffnen:** `Cmd/Ctrl + Shift + W`
* **Views minimieren / wiederherstellen:**
  konfigurierbar (empfohlen: `Cmd/Ctrl + Shift + S`)

> Hinweis: Ein Shortcut kann nicht doppelt belegt werden.

---

## Build & Release (MSI via Tags)

### CI

* Windows Builds laufen über **GitHub Actions**
* Runner: `windows-latest`

### Release erstellen

1. Version im Projekt aktualisieren (z. B. `0.1.2`)
2. Commit & Push auf `main`
3. Tag erstellen und pushen:

```bash
git tag v0.1.2
git push origin v0.1.2
```

**Ergebnis**

* GitHub Release `Wallboard v0.1.2`
* MSI als Release Asset

---

## Troubleshooting

### Settings öffnen sich, sind aber nicht sichtbar

Ursache:
View-Fenster sind `always_on_top`.

Lösung:

* Settings-Fenster ebenfalls `always_on_top`
* Fokus setzen & zentrieren (im Projekt vorgesehen)

---

### „Apply“ wirkt erst nach Neustart

* `Apply` muss bestehende Fenster **idempotent** aktualisieren
* Umsetzung via `navigate + retile`
* Kein Close/Recreate → keine Race Conditions

---

### „Label already exists“

* Fenster wird doppelt erzeugt
* Lösung: bestehendes Fenster **updaten**, nicht neu erstellen

---

### DevTools liegen im Hintergrund

* `always_on_top` kann DevTools verdecken
* Workaround:

  * beim Öffnen der Settings
  * Views temporär nicht `always_on_top` setzen

---

## Projektstruktur

```
src-tauri/
  src/
    main.rs              # App Setup, Commands, Plugins
    app_config.rs        # config.json laden / erzeugen
    window_manager.rs    # 4 Windows erstellen, kacheln, apply
    settings_api.rs      # get / save / list / apply config
    settings_window.rs   # Settings-Fenster öffnen / fokussieren

src/
  main.tsx               # Global Shortcuts (Settings / Minimize)
  settings/              # Settings UI (HTML / CSS / TS)

.github/workflows/
  release-windows-msi.yml
```

---

## Issues: Bugs & Feature Requests

Bitte GitHub Issues verwenden.

### 🐞 Bug Report

Enthalten:

* Beschreibung (Ist-Zustand vs. Erwartung)
* Schritte zum Reproduzieren
* Plattform (macOS / Windows, Version)
* Logs
* Relevanter Ausschnitt aus `config.json` (ohne Credentials)

**Titel-Format**

```
bug: settings window not visible on top
bug: apply_config does not update view 2 url
```

---

### ✨ Feature Request

Enthalten:

* Problem / Use-Case
* Lösungsvorschlag
* Optional: UI/UX-Skizze

**Titel-Format**

```
feat: fullscreen single view
feat: reload individual view
feat: autostart on windows
```

### Labels

* `bug`
* `feature`
* `enhancement`
* `windows`
* `macos`
* `kiosk`

---

## Security

* **Keine Credentials** in `config.json`
* Sessions über WebView-Profile (Cookies / Storage)
* Optional (zukünftig):

  * OS Keychain
  * Windows Credential Manager

---

## Roadmap (kurz)

* Settings UI finalisieren (Validierung, UX)
* Tray / Autostart / Kiosk-Mode (optional)
* Logging to file
* „Reload View“ & „Fullscreen View“
* Code Signing für MSI (Enterprise)

---

## Lizenz

MIT

```
```
