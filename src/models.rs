use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

/// Represents actions that can be triggered in the user interface.
///
/// The `Action` enum defines various actions that can be triggered by the user in the user interface.
/// These actions include navigation to Epic or Story details, creation, deletion, and updates of Epics
/// and Stories, navigation to the previous page, and exiting the application.
///
/// # Examples
///
/// ```
/// use crate::models::Action;
///
/// let action = Action::NavigateToEpicDetail { epic_id: 1 };
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    /// Navigate to the detail page of an Epic identified by its ID.
    NavigateToEpicDetail { epic_id: u32 },

    /// Navigate to the detail page of a Story within an Epic identified by their IDs.
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },

    /// Navigate to the previous page in the user interface.
    NavigateToPreviousPage,

    /// Create a new Epic.
    CreateEpic,

    /// Update the status of an Epic identified by its ID.
    UpdateEpicStatus { epic_id: u32 },

    /// Delete an Epic identified by its ID.
    DeleteEpic { epic_id: u32 },

    /// Create a new Story within an Epic identified by its ID.
    CreateStory { epic_id: u32 },

    /// Update the status of a Story identified by its ID.
    UpdateStoryStatus { story_id: u32 },

    /// Delete a Story within an Epic identified by their IDs.
    DeleteStory { epic_id: u32, story_id: u32 },

    /// Exit the application.
    Exit,
}

/// Represents the status of an Epic or a Story.
///
/// The `Status` enum defines various statuses that can be assigned to an Epic or a Story.
/// These statuses include Open, InProgress, Resolved, and Closed.
///
/// # Examples
///
/// ```
/// use crate::models::Status;
///
/// let status = Status::Open;
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Status {

    /// Indicates that an Epic or a Story is in an open state.
    Open,

    /// Indicates that an Epic or a Story is in progress.
    InProgress,

    /// Indicates that an Epic or a Story is resolved.
    Resolved,

    /// Indicates that an Epic or a Story is closed.
    Closed
}

/// Formats the `Status` enum variant for display.
///
/// This method implements the `fmt` function from the `std::fmt::Display` trait for the `Status` enum.
/// It formats the `Status` variant as a string representation suitable for display.
///
/// # Arguments
///
/// * `f` - A mutable reference to the formatter.
///
/// # Returns
///
/// Returns a `std::fmt::Result` indicating the success or failure of the formatting operation.
///
/// # Examples
///
/// ```
/// use crate::models::Status;
///
/// let status = Status::Open;
/// println!("{}", status); // Prints "OPEN"
/// ```
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open =>  write!(f, "OPEN"),
            Self::InProgress =>  write!(f, "IN PROGRESS"),
            Self::Resolved => write!(f, "RESOLVED"),
            Self::Closed => write!(f, "CLOSED")
        }
    }
}

/// Represents an Epic in the JIRA-like CLI tool.
///
/// The `Epic` struct represents an Epic within the JIRA-like CLI tool. It contains fields for
/// the name, description, status, and a list of story IDs associated with the Epic.
///
/// # Examples
///
/// ```
/// use crate::models::{Epic, Status};
///
/// let epic = Epic {
///     name: "Epic Name".to_string(),
///     description: "Epic Description".to_string(),
///     status: Status::Open,
///     stories: vec![1, 2, 3],
/// };
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>
}

impl Epic {

    /// Constructs a new `Epic` instance.
    ///
    /// This method creates a new `Epic` instance with the provided name and description.
    /// The status of the Epic is set to `Status::Open` by default, and the list of associated
    /// story IDs is initialized as an empty vector.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Epic.
    /// * `description` - The description of the Epic.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::models::{Epic, Status};
    ///
    /// let epic = Epic::new("Epic Name".to_string(), "Epic Description".to_string());
    /// ```
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
    }
}

/// Represents a Story in the JIRA-like CLI tool.
///
/// The `Story` struct represents a Story within the JIRA-like CLI tool. It contains fields for
/// the name, description, and status of the Story.
///
/// # Examples
///
/// ```
/// use crate::models::{Story, Status};
///
/// let story = Story {
///     name: "Story Name".to_string(),
///     description: "Story Description".to_string(),
///     status: Status::Open,
/// };
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {

    /// Constructs a new `Story` instance.
    ///
    /// This method creates a new `Story` instance with the provided name and description.
    /// The status of the Story is set to `Status::Open` by default.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Story.
    /// * `description` - The description of the Story.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::models::{Story, Status};
    ///
    /// let story = Story::new("Story Name".to_string(), "Story Description".to_string());
    /// ```
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}


/// Represents the state of the database in the JIRA-like CLI tool.
///
/// The `DBState` struct represents the state of the database within the JIRA-like CLI tool.
/// It contains fields to keep track of the last item ID to create new IDs, as well as HashMaps
/// for storing Epics and Stories with their respective IDs as keys.
///
/// # Examples
///
/// ```
/// use crate::models::{DBState, Epic, Story};
/// use std::collections::HashMap;
///
/// let db_state = DBState {
///     last_item_id: 1,
///     epics: HashMap::new(),
///     stories: HashMap::new(),
/// };
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DBState {

    /// Keeps track of the last item ID to create new IDs.
    pub last_item_id: u32,

    /// HashMap storing Epics with their IDs as keys.
    pub epics: HashMap<u32, Epic>,

    /// HashMap storing Stories with their IDs as keys.
    pub stories: HashMap<u32, Story>
}