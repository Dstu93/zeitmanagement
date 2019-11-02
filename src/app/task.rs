use chrono::{DateTime, Utc};
use serde::{Deserialize,Serialize};

#[derive(PartialOrd, PartialEq,Clone,Debug,Hash,Serialize,Deserialize)]
pub struct Task {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub comment: Option<String>,
}

impl Task {
    pub fn stop(&mut self){
        self.end = Some(Utc::now());
    }
}


