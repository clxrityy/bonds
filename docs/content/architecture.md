# Bonds | Architecture

> This document provides an overview of the architecture of the Bonds project, including the main components, their interactions, and how they work together to provide the functionality of the tool. The architecture is designed to be modular and extensible, allowing for future enhancements and integrations.

- [High-Level Architecture](#high-level-architecture)
- [Detailed Sequence Diagram](#detailed-sequence-diagram)
- [Config Management](#config-management)

---

## High-Level Architecture

A high-level overview of the main components and their relationships. This is intentionally simple to keep it readable on mobile devices. See the detailed sequence diagram below for more specifics on interactions.

```mermaid
flowchart TB
    %% High-level relationship only to keep the graph narrow on mobile.
    user["User / Shell"] --> cli["bonds-cli"]
    cli -->|invokes| core["bonds-core"]
    core -->|reads/writes| storage["Persistence + OS"]

    %% Optional direct CLI reads (for defaults/config UX).
    cli -->|reads defaults| storage

    classDef cli fill:#4C8DFF22,stroke:#4C8DFF,stroke-width:1.8px,color:#4C8DFF;
    classDef core fill:#34B27B22,stroke:#34B27B,stroke-width:1.8px,color:#34B27B;
    classDef data fill:#E09A3E22,stroke:#E09A3E,stroke-width:1.8px,color:#E09A3E;

    class cli cli
    class core core
    class storage data
```

## Detailed Sequence Diagram

This diagram shows a more detailed sequence of interactions for the `bond add` command, including filesystem and database operations.

```mermaid
%%{init: {
  "theme": "base",
  "themeVariables": {
    "fontFamily": "Inter, system-ui, sans-serif",
    "fontSize": "14px",
    "actorBkg": "#EAF2FF",
    "actorBorder": "#4C8DFF",
    "actorTextColor": "#1E3A8A",
    "actorLineColor": "#4C8DFF",
    "signalColor": "#374151",
    "signalTextColor": "#0051ff",
    "labelBoxBkgColor": "#F3F4F6",
    "labelBoxBorderColor": "#9CA3AF",
    "labelTextColor": "#111827",
    "noteBkgColor": "#FFF7ED",
    "noteBorderColor": "#FB923C",
    "noteTextColor": "#7C2D12",
    "activationBkgColor": "#E0E7FF",
    "activationBorderColor": "#6366F1"
  },
  "sequence": {
    "mirrorActors": false,
    "showSequenceNumbers": true,
    "messageMargin": 28,
    "diagramMarginY": 18,
    "diagramMarginX": 16,
    "boxMargin": 8,
    "boxTextMargin": 6,
    "actorMargin": 40,
    "width": 210,
    "height": 70,
    "noteMargin": 10,
    "leftMargin": 16,
    "rightMargin": 16,
    "wrap": true,
    "wrapPadding": 10
  }
}}%%
sequenceDiagram
    %% Example style controls for your bond add flow.
    actor U as User
    participant C as bond CLI
    participant M as BondManager
    participant FS as Filesystem
    participant DB as SQLite

    U->>C: bond add <source> [target]
    C->>FS: canonicalize / resolve paths
    C->>M: create_bond(source, target, name)
    activate M
    M->>FS: create symlink
    M->>DB: INSERT bond record
    deactivate M
    M-->>C: Bond result
    C-->>U: success or formatted error

    Note over C,M: Tune themeVariables + sequence config above
```

## Config Management

Config management is a core concern but is kept separate from the main architecture graph to avoid clutter. The `BondsConfig` component handles reading/writing the config file and resolving defaults, which is used by both the CLI and core components.

```mermaid
flowchart TB
    %% Keep config concerns separate so the main architecture graph stays uncluttered.
    cfg_cmd["bond config get/set"] --> cfg["BondsConfig"]
    add_cmd["bond add (without target)"] --> cfg
    cfg --> toml["~/.bonds/config.toml"]
    cfg --> def["default_target"]
    def --> resolved["resolved target path"]

    classDef core fill:#34B27B22,stroke:#34B27B,stroke-width:1.8px,color:#34B27B;
    classDef data fill:#E09A3E22,stroke:#E09A3E,stroke-width:1.8px,color:#E09A3E;

    class cfg core
    class toml,def,resolved data
```
