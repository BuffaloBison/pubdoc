# Gemini Clear Browser (Linux)

A privacy-focused web browser for Linux, built with Rust using GTK 3 and WebKit2GTK. This is a port of the macOS Swift app **Gemini_browser_clear.app**.

## Features

- **Web Browsing** — Full web rendering powered by WebKit2GTK
- **Address Bar** — Enter URLs and press Enter or click Go
- **Back / Forward** — Standard navigation buttons
- **CLEAR Button** — Wipes cookies, disk cache, local storage, memory cache, and offline application cache, then shows a confirmation page
- **Privacy-First Homepage** — Defaults to [StartPage](https://www.startpage.com)

## Prerequisites

Install the required system libraries.

**Pop!_OS 22.04 / Ubuntu 22.04 (Debian-based):**

```bash
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.1-dev
```

**Fedora / RHEL:**

```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

Arch Linux:

```bash
sudo pacman -S gtk3 webkit2gtk-4.1
```

## Build & Run

```bash
cd gemini_clear_browser_linux
cargo build --release
cargo run --release
```

The browser window will open with StartPage as the homepage.

## Usage

| Action | How |
|---|---|
| Navigate | Type a URL in the address bar and press **Enter** or click **Go** |
| Go Back | Click **◀** |
| Go Forward | Click **▶** |
| Clear Data | Click **CLEAR** — removes cookies, cache, and local storage |
