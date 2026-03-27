#![forbid(unsafe_code)]

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupId(pub Vec<u8>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalKind {
    Add,
    Update,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupChangeIntent {
    pub group_id: GroupId,
    pub epoch: Epoch,
    pub proposals: Vec<ProposalKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    NotImplemented(&'static str),
    ValidationFailed(&'static str),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotImplemented(message) => write!(f, "not implemented: {message}"),
            Self::ValidationFailed(message) => write!(f, "validation failed: {message}"),
        }
    }
}

impl std::error::Error for CoreError {}
