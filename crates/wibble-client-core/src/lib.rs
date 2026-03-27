#![forbid(unsafe_code)]

pub use wibble_core_domain::{CoreError, Epoch, GroupChangeIntent, GroupId, ProposalKind};

use wibble_openmls_engine::OpenMlsEngine;

#[derive(Debug, Default)]
pub struct ClientCore {
    engine: OpenMlsEngine,
}

impl ClientCore {
    pub fn new() -> Self {
        Self {
            engine: OpenMlsEngine::new(),
        }
    }

    pub fn validate_change_intent(&self, change: &GroupChangeIntent) -> Result<(), CoreError> {
        self.engine.validate_change_intent(change)
    }

    pub fn apply_change_intent(&self, change: &GroupChangeIntent) -> Result<(), CoreError> {
        self.engine.apply_change_intent(change)
    }
}

pub fn bootstrap_client_core() -> ClientCore {
    ClientCore::new()
}
