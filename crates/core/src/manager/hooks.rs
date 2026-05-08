use super::*;

/// Hook registration and dispatch concerns.
impl BondManager {
    /// Register closure/type-based hook.
    pub fn register_hook<H>(&self, hook: H)
    where
        H: BondEventHook + 'static,
    {
        self.register_hook_arc(Arc::new(hook));
    }

    /// Register Arc-wrapped hook implementation.
    pub fn register_hook_arc(&self, hook: Arc<dyn BondEventHook>) {
        if let Ok(mut hooks) = self.hooks.write() {
            hooks.push(hook);
        }
    }

    /// Emit event to all hooks.
    ///
    /// `pub(super)` is important: sibling modules (lifecycle/health) call this.
    pub(super) fn emit_event(&self, payload: BondEventPayload) {
        let hooks = match self.hooks.read() {
            Ok(h) => h.clone(),
            // If lock is poisoned, skip dispatch but do not fail core operation.
            Err(_) => return,
        };

        if hooks.is_empty() {
            return;
        }

        let event = BondEvent {
            occurred_at: Utc::now(),
            payload,
        };

        for hook in hooks {
            hook.on_event(&event);
        }
    }
}
