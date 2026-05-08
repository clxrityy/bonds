# Bonds | Events

[`bonds-core`](https://bonds.fyi/latest/api/bonds_core/) provides an in-process hooks system for bond lifecycle notifications.

## Event kinds

`BondEventKind` includes:

- `Created`
- `Updated`
- `Deleted`
- `MetadataUpdated`
- `BrokenDetected`

The full event payload is carried in `BondEventPayload`.

## Registering a hook

```rust
use std::sync::{Arc, Mutex};

use bonds_core::{BondError, BondEvent, BondEventPayload, BondManager};

let manager = BondManager::new(None)?;

// Shared sink so the hook can capture event labels for later inspection.
let labels: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
let labels_for_hook = Arc::clone(&labels);

manager.register_hook(move |event: &BondEvent| {
    // Hooks are synchronous; keep callback logic short and non-blocking.
    let label = match &event.payload {
        BondEventPayload::Created { .. } => "created",
        BondEventPayload::Updated { .. } => "updated",
        BondEventPayload::Deleted { .. } => "deleted",
        BondEventPayload::MetadataUpdated { .. } => "metadata_updated",
        BondEventPayload::BrokenDetected { .. } => "broken_detected",
    };

    labels_for_hook
        .lock()
        .expect("labels lock")
        .push(label.to_string());
});
# Ok::<(), BondError>(())
```

## Detecting broken bonds

```rust
use bonds_core::{BondError, BondEvent, BondEventPayload, BondManager};

let manager = BondManager::new(None)?;

manager.register_hook(|event: &BondEvent| {
    // React only to health-related events.
    if let BondEventPayload::BrokenDetected { bond, reason } = &event.payload {
        eprintln!("broken bond: {} ({reason:?})", bond.id());
    }
});

// Triggers BrokenDetected events for each broken bond.
let broken = manager.scan_broken_bonds()?;
eprintln!("{} broken bond(s)", broken.len());
# Ok::<(), BondError>(())
```

## Operational notes

- Hooks are invoked synchronously in registration order.
- Hook delivery is best-effort (core operations remain successful even if hook dispatch cannot proceed).
- Keep hook handlers fast; move heavy work to your own async/background pipeline.
