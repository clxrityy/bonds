use crate::bond::Bond;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// High-level event categories for bond lifecycle changes.
/// **Example usage:**
/// ```rust
/// let event = BondEvent { ... };
/// match event.kind() {
///     BondEventKind::Created => println!("Bond was created"),
///     BondEventKind::Updated => println!("Bond was updated"),
///     BondEventKind::Deleted => println!("Bond was deleted"),
///     BondEventKind::MetadataUpdated => println!("Bond metadata was updated"),
///     BondEventKind::BrokenDetected => println!("Bond is broken"),
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BondEventKind {
    Created,
    Updated,
    Deleted,
    MetadataUpdated,
    BrokenDetected,
}

/// Why a bond was considered broken during health scanning.
/// **Example usage:**
/// ```rust
/// let reason = BondBrokenReason::MissingTarget;
/// let reason = BondBrokenReason::TargetNotSymlink;
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BondBrokenReason {
    /// Target path is missing or the symlink points to a missing location.
    MissingTarget,
    /// Target exists but is not a symlink.
    TargetNotSymlink,
}

/// Detailed payload for each event kind.
/// **Example usage:**
/// ```rust
/// let payload = BondEventPayload::Created { bond: my_bond.clone() };
/// let payload = BondEventPayload::Updated { before: old_bond.clone(), after: new_bond.clone() };
/// let payload = BondEventPayload::Deleted { bond: my_bond.clone() };
/// let payload = BondEventPayload::MetadataUpdated { before: old_bond.clone(), after: new_bond.clone() };
/// let payload = BondEventPayload::BrokenDetected { bond: my_bond.clone(), reason: BondBrokenReason::MissingTarget };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BondEventPayload {
    Created {
        bond: Bond,
    },
    Updated {
        before: Bond,
        after: Bond,
    },
    Deleted {
        bond: Bond,
    },
    MetadataUpdated {
        before: Bond,
        after: Bond,
    },
    BrokenDetected {
        bond: Bond,
        reason: BondBrokenReason,
    },
}

/// Event envelope with timestamp and typed payload.
/// **Example usage:**
/// ```rust
/// let event = BondEvent {
///     occurred_at: Utc::now(),
///     payload: BondEventPayload::Created { bond: my_bond.clone() },
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BondEvent {
    pub occurred_at: DateTime<Utc>,
    pub payload: BondEventPayload,
}

impl BondEvent {
    /// Convenience accessor so callers can switch on kind quickly.
    pub fn kind(&self) -> BondEventKind {
        match self.payload {
            BondEventPayload::Created { .. } => BondEventKind::Created,
            BondEventPayload::Updated { .. } => BondEventKind::Updated,
            BondEventPayload::Deleted { .. } => BondEventKind::Deleted,
            BondEventPayload::MetadataUpdated { .. } => BondEventKind::MetadataUpdated,
            BondEventPayload::BrokenDetected { .. } => BondEventKind::BrokenDetected,
        }
    }
}

/// Hook trait for consumers that want lifecycle notifications.
/// **Example usage:**
/// ```rust
/// struct MyHook;
/// impl BondEventHook for MyHook {
///     fn on_event(&self, event: &BondEvent) {
///         println!("Received event: {:?}", event);
///     }
/// }
/// let hook = MyHook;
/// manager.register_hook(Box::new(hook));
/// ```
pub trait BondEventHook: Send + Sync {
    fn on_event(&self, event: &BondEvent);
}

/// Allow closures/functions to be registered directly as hooks.
impl<F> BondEventHook for F
where
    F: Fn(&BondEvent) + Send + Sync,
{
    fn on_event(&self, event: &BondEvent) {
        (self)(event);
    }
}
