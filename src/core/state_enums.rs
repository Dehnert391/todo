use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub enum TodoItemState {
    Open,
    Closed,
    Blocked,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityState {
    Height,
    Middle,
    Low,
}