use super::wayland::InputMethodState;
use tracing::{debug, warn};

#[derive(Debug)]
pub enum PendingOp<'a> {
    Push(&'a str),
    Pop(char),
    Clear(&'a str),
    Commit(&'a str),
    Observe(&'a str),
}

pub fn log_pending(state: &InputMethodState, op: PendingOp) {
    let chars: String = state.pending_chars.iter().collect();
    match op {
        PendingOp::Push(s) => debug!(
            "CHAR push {:?} | pending={:?} | own={} serial={} cause={:?}",
            s, chars, state.pending_own_commits, state.serial, state.pending_text_change_cause
        ),
        PendingOp::Pop(c) => debug!(
            "CHAR pop {:?} | pending={:?} | own={}",
            c, chars, state.pending_own_commits
        ),
        PendingOp::Clear(reason) => {
            if !state.pending_chars.is_empty() {
                warn!(
                    "CHAR CLEAR ({}) | lost={:?} | own={} serial={} cause={:?} last_commit_age={:?}",
                    reason,
                    chars,
                    state.pending_own_commits,
                    state.serial,
                    state.pending_text_change_cause,
                    state.last_own_commit_at.map(|t| t.elapsed()),
                );
            }
        }
        PendingOp::Commit(t) => debug!(
            "CHAR commit | text={:?} | pending={:?} | own={} serial={}",
            t, chars, state.pending_own_commits, state.serial
        ),
        PendingOp::Observe(tag) => debug!(
            "CHAR observe ({}) | pending={:?} | own={} serial={} cause={:?}",
            tag, chars, state.pending_own_commits, state.serial, state.pending_text_change_cause
        ),
    }
}

pub fn clear_pending(state: &mut InputMethodState, reason: &'static str) {
    if !state.pending_chars.is_empty() {
        log_pending(state, PendingOp::Clear(reason));
        state.pending_chars.clear();
    }
}
