#![forbid(unsafe_code)]

use wibble_core_domain::{CoreError, GroupChangeIntent};

#[derive(Debug, Default)]
pub struct OpenMlsEngine;

impl OpenMlsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_change_intent(&self, _change: &GroupChangeIntent) -> Result<(), CoreError> {
        Ok(())
    }

    pub fn apply_change_intent(&self, _change: &GroupChangeIntent) -> Result<(), CoreError> {
        Err(CoreError::NotImplemented(
            "OpenMLS integration will be added in phase 1",
        ))
    }
}
