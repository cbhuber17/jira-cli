use std::collection::HashMap;

pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>
}

impl Epic {

    // Epic constructor
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {

    // Story constructor
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

// This struct represents the entire db state
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}