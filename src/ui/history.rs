//! Undo / redo for the project model.
//!
//! Snapshot based: the whole `Ximod` is cheap enough to clone once per edit
//! burst (a few kilobytes for a typical installer, a few hundred kilobytes
//! for the largest ones), and snapshots make every editing path undoable
//! without touching the ninety call sites that mutate the model.
//!
//! Bursts: consecutive edits within [`COALESCE_SECS`] (typing in a text
//! field) are merged into one history entry, so Ctrl+Z undoes a word, not a
//! keystroke. The `base` field holds the model as it was before the current
//! burst; it is pushed to the undo stack when the burst starts and refreshed
//! when it ends.

use crate::models::Ximod;

/// Edits closer than this (seconds) are merged into one undo step.
pub const COALESCE_SECS: f64 = 0.8;
/// Maximum number of undo steps kept per document.
pub const MAX_STEPS: usize = 60;

#[derive(Clone, Default)]
pub struct History {
    undo: Vec<Ximod>,
    redo: Vec<Ximod>,
    /// The model as it was before the burst in progress (or the last known
    /// clean state when no burst is open).
    base: Option<Ximod>,
    /// Time of the last edit of the open burst, if any.
    burst_at: Option<f64>,
}

impl History {
    /// Forget everything and start from `current` (after load / new / close).
    pub fn reset(&mut self, current: &Ximod) {
        self.undo.clear();
        self.redo.clear();
        self.base = Some(current.clone());
        self.burst_at = None;
    }

    /// Record that the model was just edited. `now` is the frame time.
    ///
    /// Must be called *after* the mutation, with the time of the edit. The
    /// pre-edit state is taken from `base`, which is why `end_burst` must run
    /// (from `update`) once the burst is over.
    pub fn record_edit(&mut self, now: f64) {
        match self.burst_at {
            Some(t) if now - t < COALESCE_SECS => {
                // Same burst: keep the base, extend the burst.
                self.burst_at = Some(now);
            }
            _ => {
                // New burst: the base is what the user will come back to.
                if let Some(base) = self.base.take() {
                    self.undo.push(base);
                    if self.undo.len() > MAX_STEPS {
                        self.undo.remove(0);
                    }
                }
                self.redo.clear();
                self.burst_at = Some(now);
            }
        }
    }

    /// Close the burst when enough time has passed, refreshing the base from
    /// the current model. Call once per frame; cheap when nothing is open.
    pub fn end_burst(&mut self, now: f64, current: &Ximod) {
        if let Some(t) = self.burst_at
            && now - t >= COALESCE_SECS
        {
            self.base = Some(current.clone());
            self.burst_at = None;
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty() || self.burst_at.is_some()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Undo: returns the model to restore. `current` is pushed to redo.
    pub fn undo(&mut self, current: &Ximod) -> Option<Ximod> {
        // An open burst means `base` was already pushed; close it first so the
        // in-progress edit is itself undoable as one step.
        self.burst_at = None;
        let previous = self.undo.pop()?;
        self.redo.push(current.clone());
        self.base = Some(previous.clone());
        Some(previous)
    }

    /// Redo: returns the model to restore. `current` is pushed to undo.
    pub fn redo(&mut self, current: &Ximod) -> Option<Ximod> {
        self.burst_at = None;
        let next = self.redo.pop()?;
        self.undo.push(current.clone());
        self.base = Some(next.clone());
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(name: &str) -> Ximod {
        Ximod::new(name)
    }

    #[test]
    fn edits_in_a_burst_are_one_step() {
        let mut h = History::default();
        let mut m = model("a");
        h.reset(&m);
        m.name = "ab".into();
        h.record_edit(0.0);
        m.name = "abc".into();
        h.record_edit(0.3);
        h.end_burst(2.0, &m);
        assert!(h.can_undo());
        let back = h.undo(&m).unwrap();
        assert_eq!(back.name, "a", "one undo reverts the whole burst");
        assert!(h.can_redo());
        let fwd = h.redo(&back).unwrap();
        assert_eq!(fwd.name, "abc");
    }

    #[test]
    fn separate_bursts_are_separate_steps() {
        let mut h = History::default();
        let mut m = model("a");
        h.reset(&m);
        m.name = "b".into();
        h.record_edit(0.0);
        h.end_burst(5.0, &m);
        m.name = "c".into();
        h.record_edit(5.0);
        h.end_burst(10.0, &m);
        assert_eq!(h.undo(&m).unwrap().name, "b");
        let m2 = model("b");
        assert_eq!(h.undo(&m2).unwrap().name, "a");
        assert!(!h.can_undo());
    }

    #[test]
    fn a_new_edit_clears_redo() {
        let mut h = History::default();
        let mut m = model("a");
        h.reset(&m);
        m.name = "b".into();
        h.record_edit(0.0);
        h.end_burst(5.0, &m);
        let back = h.undo(&m).unwrap();
        assert!(h.can_redo());
        let mut m = back;
        m.name = "z".into();
        h.record_edit(6.0);
        assert!(!h.can_redo());
    }

    #[test]
    fn stack_is_bounded() {
        let mut h = History::default();
        let mut m = model("0");
        h.reset(&m);
        for i in 1..(MAX_STEPS + 20) {
            m.name = i.to_string();
            h.record_edit(i as f64 * 10.0);
            h.end_burst(i as f64 * 10.0 + 5.0, &m);
        }
        assert_eq!(h.undo.len(), MAX_STEPS);
    }
}
