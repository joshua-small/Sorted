# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Attribution

Sorted was extracted as a standalone project from the WaterBallSortPuzzleOptimalSolver codebase.

- Extracted from: [/joshua-small/WaterBallSortPuzzleOptimalSolver](https://github.com/joshua-small/WaterBallSortPuzzleOptimalSolver)
- Original upstream project: [/hkociemba/WaterBallSortPuzzleOptimalSolver](https://github.com/hkociemba/WaterBallSortPuzzleOptimalSolver)
- Original author: Herbert Kociemba, with contributors
- Extraction date: 2026-03-29

This repository keeps building on that foundation as an independently maintained project focused on the Sorted app experience.

### Provenance Note

This repository contains the contents of the former sorted subdirectory, promoted to repository root and republished as a standalone project.

## iPhone Development Testing

While the game is in active development, the fastest way to test on iPhone is the web build (Safari + Add to Home Screen).

1. Start the local preview server on your LAN:

```bash
npm run preview:mobile
```

1. On iPhone, open `http://<your-computer-ip>:4173` in Safari.
1. Optional: use Share -> Add to Home Screen for an app-like fullscreen launch.

The app includes a web manifest at `static/site.webmanifest` and Apple web-app meta tags in `src/app.html` for home screen install behavior.
