<div align="center">

# Rustroom

**A non-destructive photo editor built with Rust, Tauri and Svelte, with AI features on the roadmap.**

![Status](https://img.shields.io/badge/status-early%20development-orange)
![Rust](https://img.shields.io/badge/Rust-2021-b7410e?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-2.x-24c8db?logo=tauri)
![Svelte](https://img.shields.io/badge/Svelte-TypeScript-ff3e00?logo=svelte)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey?logo=apple)
![License](https://img.shields.io/badge/license-MIT-blue)

</div>

---

## About

Rustroom is a hobby project: a Lightroom-style photo editor that runs as a lightweight native desktop app. It is also a playground for learning Rust on a real problem, with image processing, parallelism and a clean architecture.

The core idea is **non-destructive editing**. Your original files are never touched. Rustroom stores only a small set of edit parameters per image and renders the result on demand.

> **Status:** early development. Expect rough edges and breaking changes.

## Features

**Working or in progress**
- [x] Open JPEG images
- [ ] Exposure, contrast and saturation sliders with live preview
- [ ] Non-destructive edits stored as JSON next to the image
- [ ] Full-resolution export

**Planned**
- [ ] White balance, tone curves, histogram
- [ ] Folder import, thumbnails and a SQLite catalog (ratings, tags)
- [ ] RAW support and proper colour management
- [ ] GPU-accelerated pipeline (`wgpu`)
- [ ] AI features: subject masks, auto-tagging, denoising

## Architecture

Rustroom is a single process. The Rust core owns the image pipeline and state; the TypeScript frontend only renders the UI and sends edit parameters.

```
┌────────────────────────── Rustroom.app ──────────────────────────┐
│                                                               │
│   Svelte + TypeScript UI (WebView)                            │
│        │  invoke("render_preview", params)                    │
│        ▼                                                      │
│   Rust core (Tauri)                                           │
│     ├─ Pipeline:  Exposure → Contrast → Saturation → …        │
│     ├─ Catalog:   SQLite (planned)                            │
│     └─ rayon / wgpu for performance                           │
│        │  localhost / socket (planned)                        │
│        ▼                                                      │
│   Python AI sidecar (FastAPI + ONNX/PyTorch)                  │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

**Render model:** `render(original, params) -> image` is a pure function. The same input always produces the same output, and the original is never modified. Previews are rendered from a downscaled copy; full resolution is used only for export.

## Tech stack

| Layer | Technology |
|---|---|
| Core / image pipeline | Rust (`image`, `rayon`, `serde`) |
| Desktop shell | Tauri 2 |
| Frontend | Svelte + TypeScript + Vite |
| AI (planned) | Python sidecar (FastAPI), ONNX Runtime |
| Catalog (planned) | SQLite |

## Getting started

### Prerequisites

- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) 20+
- Xcode Command Line Tools: `xcode-select --install`

### Run in development

```bash
git clone https://github.com/lsch0lz/Rustroom.git
cd Rustroom
npm install
npm run tauri dev
```

### Build a release bundle

```bash
npm run tauri build
```

The `.app` and `.dmg` end up in `src-tauri/target/release/bundle/`.

> **Tip:** Image processing is much faster in release mode. If the dev build feels sluggish, set `opt-level = 3` under `[profile.dev]` in `src-tauri/Cargo.toml`.

## Project structure

```
Rustroom/
├─ src/                 # Svelte + TypeScript frontend
├─ src-tauri/
│  ├─ src/
│  │  ├─ lib.rs         # Tauri setup and commands
│  │  └─ pipeline/      # Edit operations and render function
│  ├─ Cargo.toml
│  └─ tauri.conf.json
└─ README.md
```

## Roadmap

1. **Foundation:** pipeline, parameters, live preview, export
2. **Library:** import, thumbnails, catalog
3. **Quality:** RAW, colour management, performance (`rayon`, then `wgpu`)
4. **Intelligence:** AI sidecar with masks and auto-tagging

## Contributing

This is a personal learning project, but issues and ideas are welcome. If you want to send a pull request, please open an issue first so we can talk it through.

## License

MIT. See [LICENSE](LICENSE) for details.