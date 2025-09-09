# Repository Guidelines

## Project Structure & Module Organization
- `src/`: React + TypeScript UI (components, utils, types, assets). Example: `src/components/IdeaGenerator.tsx`.
- `src-tauri/`: Tauri (Rust) backend. Entry: `src-tauri/src/main.rs`; config: `src-tauri/tauri.conf.json`.
- `public/` and `index.html`: Static assets and Vite entry.
- Tooling: Vite (`vite.config.ts`), Tailwind (`tailwind.config.js`, `postcss.config.js`), TS config (`tsconfig.json`).

## Build, Test, and Development Commands
- Install: `npm install`.
- Desktop dev (recommended): `npm run tauri dev` — launches the Tauri app with hot reload.
- Web-only dev: `npm run dev` — Vite dev server (some Tauri APIs may be unavailable in browser).
- Build frontend: `npm run build` — type-checks (`tsc`) and builds Vite assets.
- Preview build: `npm run preview` — serves `dist/` locally.
- Desktop build: `npm run tauri build` — builds the Tauri app (Rust + frontend bundle).

## Coding Style & Naming Conventions
- TypeScript: strict mode enabled; prefer functional React components and hooks.
- Indentation: 2 spaces; camelCase for variables/functions; PascalCase for React components (e.g., `FolderSelector.tsx`).
- File layout: colocate tests and styles with components when added.
- Tailwind: prefer utility classes; keep component markup readable.
- Rust: idiomatic `rustfmt` style; one command module per file where practical.

## Testing Guidelines
- JS/TS: not set up yet. Recommend Vitest + React Testing Library. Name tests `*.test.ts` / `*.test.tsx` next to source.
- Rust: add unit tests with `#[cfg(test)]` in modules; run with `cargo test` inside `src-tauri/`.
- Target coverage: ~80% lines/branches for new/changed code.

## Agent Workflow & Auto‑Commit Policy
- Always create a new git commit immediately after making any changes to the codebase.
- Use Conventional Commits for messages (e.g., `feat: ...`, `fix: ...`, `docs: ...`, `refactor: ...`, `chore: ...`).
- Keep commits atomic and scoped to the files/changes just made. Do not amend or squash by default.
- Only stage the files you modified for that change; avoid blanket adds that include unrelated work.
- When a task spans multiple logical changes, prefer separate focused commits.
- For documentation-only updates, use `docs:`; for config/infra, use `chore:`.

## Commit & Pull Request Guidelines
- Commits: use Conventional Commits where possible (e.g., `feat: add idea generator panel`). Keep changes atomic.
- PRs: include a clear description, linked issues, and screenshots/GIFs for UI changes.
- Verification: run `npm run build` and, for desktop, `npm run tauri build` before requesting review.

## Security & Configuration Tips
- Do not hardcode secrets. API URL/model/key are managed via the Settings UI and persisted by Tauri (see `save_settings`/`load_settings`).
- Keep large/binary files out of analysis; repository walk already ignores common directories (`node_modules`, `target`, `dist`).
