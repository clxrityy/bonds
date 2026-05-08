use crate::bond::Bond;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// High-level event categories for bond lifecycle changes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BondEventKind {
    Created,
    Updated,
    Deleted,
    MetadataUpdated,
    BrokenDetected,
}

/// Why a bond was considered broken during health scanning.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BondBrokenReason {
    /// Target path is missing or the symlink points to a missing location.
    MissingTarget,
    /// Target exists but is not a symlink.
    TargetNotSymlink,
}

/// Detailed payload for each event kind.
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
