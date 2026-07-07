use chrono::prelude::{Utc};
use crate::core::state_enums::{TodoItemState, PriorityState};



/// 
/// Allgemeine überlegungen: manchmal ergeben sich unter den Todo abhängigkeiten untereinander, vielleicht ist es nützlich dies zu verknüpfen.
/// 

#[derive(Clone, Debug)]
pub struct TodoItem {
    name:String,
    timestamp:chrono::DateTime<Utc>,
    deadline:Option<chrono::DateTime<Utc>>,
    forward:Option<chrono::DateTime<Utc>>, // Das soll der vorlauf sein, ich möchte die Todos so steuern können, dass sie erst zu einer bestimmten Zeit aufploppen zb 3 Tage vorlauf
    state: TodoItemState,
    priority: PriorityState,
    description:Option<String>
    //dependency = mache Todo können auch in mehrer aufgeteilt werden. daraus ergeben sich abhängigkeiten wenn z.B. Todo A eine blocker hat aber Todo B von A abhängt ist B auch geblockt.
}

impl TodoItem {
    pub fn new(name:String) -> Self {
        TodoItem { name, 
            timestamp:Utc::now(),
            deadline:None,
            forward:None,
            state:TodoItemState::Open,
            priority:PriorityState::Low,
            description:None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn timestamp(&self) -> &chrono::DateTime<Utc> {
        &self.timestamp
    }   

    pub fn forward(&self) -> &Option<chrono::DateTime<Utc>> {
        match &self.forward {
            Some(_) => &self.forward,
            None => &None,
        }
    } 

    pub fn deadline(&self) -> &Option<chrono::DateTime<Utc>> {
        match &self.deadline {
            Some(_) => &self.deadline,
            None => &None,
        }
    } 

    pub fn state(&self) -> &TodoItemState {
        &self.state
    }

    pub fn set_state(&mut self, state:&str) -> &mut Self{
        match state {
            "open" => self.state = TodoItemState::Open,
            "closed" => self.state = TodoItemState::Closed,
            "blocked" => self.state = TodoItemState::Blocked,
            _ => (),
        }
        self
    }

    pub fn set_priority_state(&mut self, state:&str) -> &mut Self {
        match state {
            "height" => self.priority = PriorityState::Height, 
            "middle" => self.priority = PriorityState::Middle,
            "low" => self.priority = PriorityState::Low,
            _ => (),
        }   
        self
    }

}