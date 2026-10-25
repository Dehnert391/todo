use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityState {
    High,
    Middle,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TodoState {
    Open,
    Running,
    Close,
    Blocked,
}

impl TodoState {
    pub fn switch_state(&mut self) {
        *self = match *self {
            Self::Open => Self::Running,
            Self::Running => Self::Close,
            Self::Close => Self::Blocked,
            Self::Blocked => Self::Open,
        }
    }

    pub fn get_label(&self) -> String {
        match self {
            Self::Open => "Open".to_string(),
            Self::Running => "Running".to_string(),
            Self::Close => "Close".to_string(),
            Self::Blocked => "Blocked".to_string(),
        }
    }
}
