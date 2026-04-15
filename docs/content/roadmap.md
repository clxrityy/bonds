---
title: Bonds | Roadmap
summary: A high-level overview of the planned features and improvements for the Bonds project.
authors:
    - MJ Anglin
    - clxrityy
    - clxrity
date: 2026-04-14
---

- [x] [Phase 1](#phase-1-cli-core-tool-): CLI (core tool)
- [ ] [Phase 2](#phase-2-library-api): Library API
- [ ] [Phase 3](#phase-3-gui-application): GUI Application
- [ ] [Phase 4](#phase-4-ecosystem--plugins): Ecosystem & Plugins

## Phase 1: CLI (core tool) ✔

The primary, usable product. Everything in this phase should work from the terminal.

- [x] Create a bond (source -> target symlink)
- [x] List bonds
- [x] Inspect bond details + health
- [x] Remove a bond (with optional target deletion)
- [x] SQLite persistence (`~/.bonds/bonds.db`)
- [x] Default target directory (e.g., `~/bonds`)
  - [x] Configurable default directory (`bond config  set default <path>`)
- [x] Bulk bonding (`bond add ~/projects/ ~/bonds --contents`)
- [x] Update / rename existing bonds
- [x] `bond migrate`: move existing bonds to a new default location (with auto-backup)
- [x] Name a bond (for easier reference and management) (displays in `bond list`)
- [x] Tests for all functionality

## Phase 2: Library API

Expose `bonds-core` & `bonds-cli` as a proper public API so developers can integrate it.

- [x] Stabilize and document the public API surface
- [ ] Add metadata read/write operations
- [ ] Hooks/events system (bond created, deleted, broken, etc.)
- [ ] Query bonds by source, target, or metadata
- [ ] API documentation + usage examples

## Phase 3: GUI Application

Tauri-based desktop app, depends on [Phase 2](#phase-2-library-api)'s API.

- [ ] Scaffold `crates/app` (Tauri)
- [ ] Bond viewer (sorted by recently updated)
  - [ ] Search/filter bonds by source, target, or metadata
- [ ] Empty-state CTA to create first bond
- [ ] Open bonded directory from UI
- [ ] Create / edit / delete bonds from UI
- [ ] Sidebar bookmarks/favorites
- [ ] Bond detail panel (source, target, metadata)
- [ ] Bond relationship visualization (graph view)

## Phase 4: Ecosystem & Plugins

Long-term extensibility goals.

- [ ] Plugin interface/framework definition
- [ ] Editor plugins (VSCode, etc.)
- [ ] Integration examples for external applications
