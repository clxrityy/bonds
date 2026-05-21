---
title: Bonds | Roadmap
summary: A staged roadmap for evolving Bonds from a symlink manager into a desktop-first workspace and history platform.
authors:
    - MJ Anglin
    - clxrityy
    - clxrity
date: 2026-05-11
---

This roadmap is organized around shipping the right layers in the right order:

1. Reach desktop workflow parity with the CLI.
2. Improve the CLI experience and desktop handoff.
3. Add a durable history engine to `bonds-core`.
4. Expose that history through the CLI and app.
5. Expand into integrations, downloads, and ecosystem tooling.

- `[x]` [Phase 1: CLI and core foundation](#phase-1-cli-and-core-foundation)
- `[x]` [Phase 2: Public library API](#phase-2-public-library-api)
- `[ ]` **[Phase 3: Desktop foundation and workflow parity](#phase-3-desktop-foundation-and-workflow-parity)**
- `[ ]` [Phase 4: CLI UX and desktop bridge](#phase-4-cli-ux-and-desktop-bridge)
- `[ ]` [Phase 5: History engine in bonds-core](#phase-5-history-engine-in-bonds-core)
- `[ ]` [Phase 6: History workflows in CLI and app](#phase-6-history-workflows-in-cli-and-app)
- `[ ]` [Phase 7: Ecosystem, integrations, and bondsfyi](#phase-7-ecosystem-integrations-and-bondsfyi)

## Guiding principles

- Keep `bonds-core` as the source of truth for behavior shared by the CLI and app.
- Ship correct history and rollback semantics before optimizing the storage format.
- Prioritize workflow parity and usability over novelty features.
- Treat the desktop app, CLI, docs, and website as one product surface.

## Phase 1: CLI and core foundation

The existing Bonds baseline is already in place.

- `[x]` Create bonds from source to target
- `[x]` List, inspect, update, migrate, and remove bonds
- `[x]` Persist bonds in SQLite (`~/.bonds/bonds.db`)
- `[x]` Support configurable defaults (`bond config`)
- `[x]` Support metadata read/write operations
- `[x]` Expose lifecycle hooks and query support
- `[x]` Maintain test coverage for current CLI/core behavior

## Phase 2: Public library API

`bonds-core` and `bonds-cli` are usable as public library surfaces.

- `[x]` Stabilize the public API surface
- `[x]` Document the library API and examples
- `[x]` Support metadata queries and lifecycle events
- `[x]` Keep the core reusable by future desktop and integration work

## Phase 3: Desktop foundation and workflow parity

This phase turns the current Tauri shell into a real desktop product.

Current baseline:

- `[x]` Scaffold `crates/app`
- `[x]` Show a bond viewer in the desktop app
- `[x]` Support status-based filtering and source/target/name search
- `[x]` Establish the retro / nostalgic design direction for the UI
- `[x]` Remove the legacy `--contents` flag from `bonds add` and update the CLI to match the new core behavior
- `[x]` Give `bond` a branded, more expressive default CLI landing output

Next milestones:

- `[x]` Create bonds from the UI
- `[ ]` Edit and delete bonds from the UI
- `[ ]` Open source and target paths from the UI
- `[ ]` Add a bond detail panel with metadata visibility/editing
- `[ ]` Add metadata-aware search and filtering
- `[ ]` Add a first-run empty state and create-first-bond flow
- `[ ]` Add a dashboard view for recent bonds and health summaries
- `[ ]` Add a file explorer view for bonded content
- `[ ]` Add a settings page for app preferences and default bond behavior
- `[ ]` Package the desktop app for installation and updates

Lower-priority desktop enhancements:

- `[ ]` Bookmarks / favorites
- `[ ]` Custom groups / tags
- `[ ]` Relationship graph view
- `[ ]` File preview polish (icons, richer previews, etc.)

## Phase 4: CLI UX and desktop bridge

This phase improves the CLI as a polished front door to the Bonds ecosystem.

- `[ ]` Improve color, layout, and readability for CLI responses
- `[ ]` Redesign `bond add` UX
- `[ ]` Document and test the migration path for the `bond add` behavior change
- `[ ]` Add `bond desktop` to launch the desktop app
- `[ ]` Prompt users to install the desktop app if it is not available
- `[ ]` Keep CLI help, docs, and implementation in sync

## Phase 5: History engine in `bonds-core`

This phase adds the primitives needed for snapshots and rollback.

### 5A. History model and restore primitives

- `[ ]` Define a durable history model for bonds, snapshots, and restore points
- `[ ]` Give every change a stable identifier and timestamp
- `[ ]` Add query APIs for history lookups by bond, file, time, and change type
- `[ ]` Add manual snapshot creation primitives
- `[ ]` Add restore / rollback primitives with safety checks
- `[ ]` Add per-bond and default history configuration

### 5B. Retention and storage strategy

- `[ ]` Add retention policies and garbage collection
- `[ ]` Introduce a storage abstraction for future optimization work
- `[ ]` Start with a correct and testable local snapshot implementation
- `[ ]` Add compression and deduplication where it clearly improves storage use
- `[ ]` Add delta encoding only after the history model and restore semantics are stable

## Phase 6: History workflows in CLI and app

Once the core history engine exists, expose it through the product surfaces.

### CLI

- `[ ]` `bond history [bond] [--limit <number>]`
- `[ ]` `bond snapshot [bond] [--name <snapshot-name>]`
- `[ ]` `bond rollback [bond] [change-id]`
- `[ ]` Support filtering by file, time range, and change type
- `[ ]` Show stable change IDs, timestamps, and rollback targets clearly

### Desktop app

- `[ ]` Add a history panel for each bond
- `[ ]` Show snapshots, changes, timestamps, and restore targets
- `[ ]` Allow one-click restore with confirmation and recovery safeguards
- `[ ]` Add scheduled snapshot configuration in settings
- `[ ]` Add file-level filtering and preview for history browsing

## Phase 7: Ecosystem, integrations, and `bonds.fyi`

This phase expands Bonds from a local tool into a broader ecosystem.

- `[ ]` Make `bonds.fyi` the central hub for docs, downloads, demos, and release notes
- `[ ]` Add desktop app download and install guidance to the site
- `[ ]` Publish deeper tutorials for CLI, core, and desktop workflows
- `[ ]` Explore cloud/offsite backup integrations
- `[ ]` Support rollback-oriented integrations for external/server data
- `[ ]` Define a plugin interface after the history layer is stable
- `[ ]` Add editor integrations such as VS Code support

## Success criteria

The roadmap is on track when the project can do all of the following:

- A user can create, inspect, edit, and delete bonds from both the CLI and desktop app.
- A user can open the desktop app from the CLI with `bond desktop`.
- A user can create snapshots and safely roll back a bond from both interfaces.
- History storage is correct first, then optimized second.
- `bonds.fyi` becomes the obvious place to learn, download, and follow the project.
