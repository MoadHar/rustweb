use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING
}
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()}
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::PENDING => {write!(f, "DONE")},
            &Self::DONE => {write!(f, "PENDING")}
        }
    }
}

impl TaskStatus {
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}
