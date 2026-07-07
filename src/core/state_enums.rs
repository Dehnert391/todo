
#[derive(Debug, Clone)]
pub enum TodoItemState {
    Open,
    Closed,
    Blocked,
}


#[derive(Debug, Clone)]
pub enum PriorityState {
    Height,
    Middle,
    Low,
}