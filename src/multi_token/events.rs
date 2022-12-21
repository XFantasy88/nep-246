//! Standard for nep246 (Non-Fungible Token) events.
//!
//! These events will be picked up by the NEAR indexer.
//!
//! This is an extension of the events format (nep-297):
//! <https://github.com/near/NEPs/blob/master/specs/Standards/EventsFormat.md>
//!
//! The three events in this standard are [`MtMint`], [`MtTransfer`], and [`MtBurn`].
//!
//! These events can be logged by calling `.emit()` on them if a single event, or calling
//! [`MtMint::emit_many`], [`MtTransfer::emit_many`],
//! or [`MtBurn::emit_many`] respectively.

use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;

/// Data to log for an MT mint event. To log this event, call [`.emit()`](MtMint::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MtMint<'a> {
    pub owner_id: &'a AccountId,
    pub token_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl MtMint<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an mt mint event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`MtMint`] represents the data of each mint.
    pub fn emit_many(data: &[MtMint<'_>]) {
        new_246_v1(Nep246EventKind::MtMint(data)).emit()
    }
}

/// Data to log for an MT transfer event. To log this event,
/// call [`.emit()`](MtTransfer::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MtTransfer<'a> {
    pub old_owner_id: &'a AccountId,
    pub new_owner_id: &'a AccountId,
    pub token_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<&'a AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl MtTransfer<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an mt transfer event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`MtTransfer`] represents the data of each transfer.
    pub fn emit_many(data: &[MtTransfer<'_>]) {
        new_246_v1(Nep246EventKind::MtTransfer(data)).emit()
    }
}

/// Data to log for an MT burn event. To log this event, call [`.emit()`](MtBurn::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct MtBurn<'a> {
    pub owner_id: &'a AccountId,
    pub token_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<&'a AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
}

impl MtBurn<'_> {
    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        Self::emit_many(&[self])
    }

    /// Emits an Mt burn event, through [`env::log_str`](near_sdk::env::log_str),
    /// where each [`MtBurn`] represents the data of each burn.
    pub fn emit_many<'a>(data: &'a [MtBurn<'a>]) {
        new_246_v1(Nep246EventKind::MtBurn(data)).emit()
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct Nep246Event<'a> {
    version: &'static str,
    #[serde(flatten)]
    event_kind: Nep246EventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Nep246EventKind<'a> {
    MtMint(&'a [MtMint<'a>]),
    MtTransfer(&'a [MtTransfer<'a>]),
    MtBurn(&'a [MtBurn<'a>]),
}

fn new_246<'a>(version: &'static str, event_kind: Nep246EventKind<'a>) -> NearEvent<'a> {
    NearEvent::Nep246(Nep246Event {
        version,
        event_kind,
    })
}

fn new_246_v1(event_kind: Nep246EventKind) -> NearEvent {
    new_246("1.0.0", event_kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils, AccountId};

    fn bob() -> AccountId {
        AccountId::new_unchecked("bob".to_string())
    }

    fn alice() -> AccountId {
        AccountId::new_unchecked("alice".to_string())
    }

    #[test]
    fn mt_mint() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        MtMint {
            owner_id,
            token_ids,
            memo: None,
        }
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep246","version":"1.0.0","event":"mt_mint","data":[{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn mt_mints() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        let mint_log = MtMint {
            owner_id,
            token_ids,
            memo: None,
        };
        MtMint::emit_many(&[
            mint_log,
            MtMint {
                owner_id: &alice(),
                token_ids: &["2", "3"],
                memo: Some("has memo"),
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep246","version":"1.0.0","event":"mt_mint","data":[{"owner_id":"bob","token_ids":["0","1"]},{"owner_id":"alice","token_ids":["2","3"],"memo":"has memo"}]}"#
        );
    }

    #[test]
    fn mt_burn() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        MtBurn {
            owner_id,
            token_ids,
            authorized_id: None,
            memo: None,
        }
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep246","version":"1.0.0","event":"mt_burn","data":[{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn mt_burns() {
        let owner_id = &bob();
        let token_ids = &["0", "1"];
        MtBurn::emit_many(&[
            MtBurn {
                owner_id: &alice(),
                token_ids: &["2", "3"],
                authorized_id: Some(&bob()),
                memo: Some("has memo"),
            },
            MtBurn {
                owner_id,
                token_ids,
                authorized_id: None,
                memo: None,
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep246","version":"1.0.0","event":"mt_burn","data":[{"owner_id":"alice","token_ids":["2","3"],"authorized_id":"bob","memo":"has memo"},{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn mt_transfer() {
        let old_owner_id = &bob();
        let new_owner_id = &alice();
        let token_ids = &["0", "1"];
        MtTransfer {
            old_owner_id,
            new_owner_id,
            token_ids,
            authorized_id: None,
            memo: None,
        }
        .emit();
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep246","version":"1.0.0","event":"mt_transfer","data":[{"old_owner_id":"bob","new_owner_id":"alice","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn mt_transfers() {
        let old_owner_id = &bob();
        let new_owner_id = &alice();
        let token_ids = &["0", "1"];
        MtTransfer::emit_many(&[
            MtTransfer {
                old_owner_id: &alice(),
                new_owner_id: &bob(),
                token_ids: &["2", "3"],
                authorized_id: Some(&bob()),
                memo: Some("has memo"),
            },
            MtTransfer {
                old_owner_id,
                new_owner_id,
                token_ids,
                authorized_id: None,
                memo: None,
            },
        ]);
        assert_eq!(
            test_utils::get_logs()[0],
            r#"EVENT_JSON:{"standard":"nep246","version":"1.0.0","event":"mt_transfer","data":[{"old_owner_id":"alice","new_owner_id":"bob","token_ids":["2","3"],"authorized_id":"bob","memo":"has memo"},{"old_owner_id":"bob","new_owner_id":"alice","token_ids":["0","1"]}]}"#
        );
    }
}
