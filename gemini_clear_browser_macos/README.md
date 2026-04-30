# Gemini Clear Browser — macOS (Rust)

A privacy-focused web browser for macOS, built with Rust using native **AppKit** and **WebKit** (WKWebView) via the [cacao](https://crates.io/crates/cacao) crate.

This is the macOS companion to the [Linux version](../gemini_clear_browser_linux/) which uses GTK 3 + WebKit2GTK.

## Features

- Native macOS window with toolbar (Back, Forward, URL bar, CLEAR)
- Full web browsing via WKWebView (Apple's WebKit engine)
- **CLEAR** button wipes cookies, cache, and local storage in one click
- Defaults to [StartPage](https://www.startpage.com) for privacy-respecting search
- Standard macOS menus (Edit, View, Window, etc.)

## Requirements

- **macOS Sequoia (15.x)** or **macOS Sonoma (14.x)**
- **Rust** (latest stable via [rustup](https://rustup.rs))
- **Xcode Command Line Tools** (for the system linker and frameworks)

## Build & Run

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **Install Xcode Command Line Tools** (if not already installed):
   ```bash
   xcode-select --install
   ```

3. **Build and run**:
   ```bash
   cd gemini_clear_browser_macos
   cargo build --release
   cargo run --release
   ```

The release binary will be at `target/release/gemini_clear_browser`.

## Architecture

| Component | Technology |
|-----------|-----------|
| GUI Framework | AppKit (native macOS) via `cacao` crate |
| Web Engine | WKWebView (native WebKit) via `cacao` webview feature |
| Data Clearing | WKWebsiteDataStore via Objective-C runtime messaging |
| Language | Rust |

## Project Structure

```
gemini_clear_browser_macos/
├── Cargo.toml           # Dependencies: cacao (AppKit + WebKit bindings), block
├── src/
│   ├── main.rs          # App delegate, window, webview, action dispatcher
│   └── toolbar.rs       # Native macOS toolbar: back, forward, URL bar, CLEAR
└── README.md
```

## Comparison with Linux Version

| | Linux | macOS |
|---|---|---|
| GUI | GTK 3 | AppKit (native) |
| Web Engine | WebKit2GTK | WKWebView (native) |
| Toolkit Crate | `gtk`, `webkit2gtk` | `cacao` |
| Look & Feel | GTK themed | Native macOS |
| Data Clearing | `WebsiteDataManager::clear()` | `WKWebsiteDataStore::removeDataOfTypes` |
