# RepoMuse

AI‑assisted repository explorer and idea generator. RepoMuse is a cross‑platform desktop app built with Tauri (Rust) and React + TypeScript that helps you:

- Point at a root folder of code projects
- Discover projects automatically (Node, Rust, Python, Go, etc.)
- Analyze a selected project (files, lines, languages, lightweight structure)
- Generate a clear project summary and actionable development ideas using an OpenAI‑compatible API (OpenAI, Azure OpenAI, DeepSeek, Ollama)
- Persist settings and summaries locally for quick revisits

## What It Does

- Project discovery: Scans the chosen root folder for likely projects using common indicators (e.g., `package.json`, `Cargo.toml`, `requirements.txt`, `.csproj`, `Makefile`). Shows name, Git presence and a fast file count that refines in the background.
- Repository analysis: Reads source files (skipping big/binary/ignored folders), detects languages by extension, and computes metrics like total files and lines plus a lightweight directory structure.
- AI summaries and ideas: Calls your configured OpenAI‑compatible endpoint to produce a human‑readable project summary and 5–10 practical, numbered development ideas. Thinking‑style models (e.g., `o1-mini`, `deepseek-r1`) are supported and hidden reasoning tags are filtered.
- Local persistence: Settings, selected root folder, file‑count cache and generated summaries are stored locally via Tauri for a snappy, privacy‑friendly experience.

## Quick Start

Prerequisites
- Node.js 18+ and npm
- Rust toolchain (stable) and platform build tools required by Tauri
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools (C++), WebView2
  - Linux: common GTK/WebKit and build packages

Install
- `npm install`

Run (desktop, recommended)
- `npm run tauri dev` — launches the Tauri app with hot reload.

Run (web‑only)
- `npm run dev` — Vite dev server in the browser. Some Tauri‑only features (e.g., native dialogs, local persistence paths) may be limited.

Build
- Frontend: `npm run build`
- Desktop app: `npm run tauri build`

## Using RepoMuse

1) Choose a root folder
- On first launch, click “Choose Folder” and select a directory that contains many project subfolders.

2) Browse projects
- The left sidebar lists detected projects. It shows a quick file count (refined asynchronously) and a Git badge when applicable.

3) Analyze a project
- Select a project to view metrics (files, lines), detected technologies, and a lightweight directory view.

4) Configure AI
- Open Settings to set `API Server URL`, optional `API Key`, and `Model`.
- Click “Load Models” to fetch available models from compatible APIs.
- Examples:
  - Ollama (local): URL `http://localhost:11434/v1/chat/completions`, empty key, models like `llama2`, `codellama`, `deepseek-r1`.
  - OpenAI: URL `https://api.openai.com/v1/chat/completions`, API key required, models like `gpt-4`, `o1-mini`.
  - DeepSeek: URL `https://api.deepseek.com/v1/chat/completions`, API key required, models like `deepseek-chat`, `deepseek-r1`.

5) Generate output
- In the Project Summary card, click “Generate Summary” to create a concise explanation of what the project is and does.
- In the Development Ideas card, click “Generate Ideas” to get actionable suggestions. Regenerate anytime.
- Summaries are saved locally and loaded automatically next time.

## Tech Stack

- Frontend: React 19, TypeScript, Vite, Tailwind CSS
- Desktop: Tauri 2 (Rust)
- Native plugins: `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-opener`
- Icons: `lucide-react`
- AI backend: Any OpenAI‑compatible Chat Completions API

## Scripts

- `npm run dev`: Start Vite dev server (web‑only)
- `npm run build`: Type‑check and build frontend
- `npm run preview`: Serve built assets from `dist/`
- `npm run tauri dev`: Run desktop app with hot reload
- `npm run tauri build`: Build production desktop binaries

## Project Structure

- `src/`: React + TypeScript UI (components, utils, types, assets)
  - Example: `src/components/ProjectAnalyzer.tsx`, `src/components/Settings.tsx`
- `src-tauri/`: Tauri (Rust) backend
  - Entry: `src-tauri/src/main.rs`
  - Config: `src-tauri/tauri.conf.json`
- `public/` and `index.html`: Static assets and Vite entry
- Tooling: `vite.config.ts`, `tailwind.config.js`, `postcss.config.js`, `tsconfig*.json`

## Privacy & Security

- No secrets are hardcoded. Enter your API URL, model and key in Settings; they are stored locally by Tauri.
- Repo content is analyzed locally; only the prompts you request (for summaries/ideas) are sent to your configured API endpoint.
- Large/binary folders like `node_modules`, `target`, and `dist` are skipped during analysis.

## Testing

- JS/TS: Not set up yet (recommended: Vitest + React Testing Library). Name tests `*.test.ts(x)` next to sources.
- Rust: Add unit tests with `#[cfg(test)]` inside modules and run with `cargo test` in `src-tauri/`.

## Troubleshooting

- Models not loading: Verify the API URL points to a models‑listing endpoint compatible with OpenAI or Ollama variants, and that your API key (if required) is valid.
- Ideas/Summary not generating: Ensure Settings are saved and the model supports chat completions. Check your network and API logs.
- Desktop build issues: Confirm Tauri prerequisites for your OS are installed and Rust toolchain is up to date.

---

Made with Tauri + React to make sense of codebases and spark next steps.
