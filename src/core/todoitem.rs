use chrono::{DateTime, NaiveDateTime, prelude::Utc};
use crate::core::state_enums::{PriorityState};
use serde::{Deserialize, Serialize, };


/// 
/// Allgemeine überlegungen: manchmal ergeben sich unter den Todo abhängigkeiten untereinander, vielleicht ist es nützlich dies zu verknüpfen.
/// 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    name:String,
    timestamp:chrono::DateTime<Utc>,
    deadline:Option<chrono::DateTime<Utc>>,
    forward:Option<chrono::DateTime<Utc>>, // Das soll der vorlauf sein, ich möchte die Todos so steuern können, dass sie erst zu einer bestimmten Zeit aufploppen zb 3 Tage vorlauf
    state: bool,
    priority: PriorityState,
    description:Option<String>,
    expand:bool,
    delete:bool,
    //dependency = manche Todo können auch in mehrer aufgeteilt werden. daraus ergeben sich abhängigkeiten wenn z.B. Todo A eine blocker hat aber Todo B von A abhängt ist B auch geblockt.
}


impl TodoItem {
    pub fn new(name:String, 
                deadline:Option<chrono::DateTime<Utc>>,
                priority: &str,
                description:&str
                ) -> Self {

        let mut todo = TodoItem { name, 
                                            timestamp:Utc::now(),
                                            deadline,
                                            forward:None,
                                            state:false,
                                            priority: PriorityState::Low,
                                            description:Some(description.to_string()),
                                            expand:false,
                                            delete:false,
                                            };
        todo.set_priority_state(&priority);
        todo
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn expand(&self) -> bool {
        self.expand
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
    
    pub fn deadline_as_string(&self) -> String {

        match &self.deadline {
            Some(_) => format!("{}", &self.deadline.unwrap().format("%d.%m.%y %H:%M")),
            None => format!("None"),
        }

    }

    pub fn delete(&self) -> bool {
        self.delete
    }
 
    pub fn set_delete_flag(&mut self, flag:bool) {
        self.delete = flag;
    }
    
    pub fn set_deadline(&mut self, time:String) {
        let fmt = "%d.%m.%y %H:%M";
        let naive: NaiveDateTime = NaiveDateTime::parse_from_str(&time, fmt).expect("");
        self.deadline =  Some(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc));
    }

    pub fn state(&self) -> bool {
        self.state
    }

    pub fn description(& self) -> String {
        match &self.description {
            Some(value) => value.trim().to_string(),
            _ => "".trim().to_string(),
        }
    }

    pub fn set_expand(&mut self, state: bool) {
        self.expand = state;
    } 

    pub fn priority(&self) -> String {
        match self.priority {
            PriorityState::High => "High".to_string(),
            PriorityState::Middle => "Middle".to_string(),
            PriorityState::Low => "Low".to_string(),
        }
    }

    pub fn priority_state(&self) -> &PriorityState {
        &self.priority
    }


    pub fn set_description(&mut self, text:String) {
        self.description = Some(text);
    }

    pub fn set_state(&mut self, state:bool) {
        self.state = state;
    }

    pub fn set_priority_state(&mut self, state:&str) -> &mut Self {

        match state.to_lowercase().as_str() {
            "high" | "hoch" => self.priority = PriorityState::High, 
            "middle" | "mittel" => self.priority = PriorityState::Middle,
            "low" | "gering" => self.priority = PriorityState::Low,
            _ => (),
        }   
        self
    }

}