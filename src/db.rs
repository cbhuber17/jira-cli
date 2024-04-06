use std::fs;
use anyhow::{anyhow, Ok, Result};
use crate::models::{DBState, Epic, Story, Status};
use colored::Colorize;

/// Trait for interacting with the database in the JIRA-like CLI tool.
///
/// The `Database` trait defines methods for reading from and writing to the database.
///
/// # Examples
///
/// ```
/// use crate::db::Database;
/// use crate::models::DBState;
/// use anyhow::Result;
///
/// struct MyDatabase;
///
/// impl Database for MyDatabase {
///     fn read_db(&self) -> Result<DBState> {
///         // Implementation for reading from the database
///         unimplemented!()
///     }
///
///     fn write_db(&self, db_state: &DBState) -> Result<()> {
///         // Implementation for writing to the database
///         unimplemented!()
///     }
/// }
/// ```
pub trait Database {
    /// Reads the database state.
    ///
    /// This method reads the state of the database and returns it as a `DBState` instance.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `DBState` instance if the read operation is successful,
    /// otherwise returns an `Err` containing an error.
    fn read_db(&self) -> Result<DBState>;

    /// Writes the database state.
    ///
    /// This method writes the provided database state to the database.
    ///
    /// # Arguments
    ///
    /// * `db_state` - The database state to be written.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the write operation.
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

/// Represents the JIRA-like database in the CLI tool.
///
/// The `JiraDatabase` struct represents the database used in the JIRA-like CLI tool.
/// It contains a field `database` which is a boxed trait object implementing the `Database` trait.
///
/// # Examples
///
/// ```
/// use crate::db::JiraDatabase;
/// use crate::db::Database;
///
/// let my_database: Box<dyn Database> = // instantiate your database implementation;
/// let jira_database = JiraDatabase { database: my_database };
/// ```
pub struct JiraDatabase {

    /// The database instance implementing the `Database` trait.
    pub database: Box<dyn Database>
}

impl JiraDatabase {

    /// Constructs a new `JiraDatabase` instance.
    ///
    /// This method creates a new `JiraDatabase` instance with the provided file path.
    /// It initializes the `database` field with a boxed instance of `JSONFileDatabase`, which
    /// implements the `Database` trait and operates on a JSON file located at the specified path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the JSON file storing the database state.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    ///
    /// let file_path = "database.json".to_string();
    /// let jira_database = JiraDatabase::new(file_path);
    /// ```
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase{file_path})
        }
    }

    /// Reads the database state.
    ///
    /// This method delegates the task of reading the database state to the underlying database
    /// implementation stored in the `database` field. It invokes the `read_db` method on the
    /// database instance and returns the result.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `DBState` instance if the read operation is successful,
    /// otherwise returns an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// match jira_database.read_db() {
    ///     Ok(db_state) => {
    ///         // Handle the retrieved database state
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }
    
    /// Creates a new Epic in the database.
    ///
    /// This method creates a new Epic in the database by inserting the provided Epic instance
    /// with an automatically generated ID. It retrieves the current database state, increments
    /// the last item ID, inserts the new Epic into the database with the generated ID, and then
    /// writes the updated state back to the database.
    ///
    /// # Arguments
    ///
    /// * `epic` - The Epic instance to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created Epic if the operation is successful,
    /// otherwise returns an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use crate::models::Epic;
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// let new_epic = Epic::new("New Epic Name".to_string(), "New Epic Description".to_string());
    /// match jira_database.create_epic(new_epic) {
    ///     Ok(epic_id) => {
    ///         // Handle the ID of the newly created Epic
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed_db = self.database.read_db()?;

        let new_id = parsed_db.last_item_id + 1;

        parsed_db.last_item_id = new_id;
        parsed_db.epics.insert(new_id, epic);

        self.database.write_db(&parsed_db)?;
        Ok(new_id)
    }
    
    /// Creates a new Story in the database and associates it with an Epic.
    ///
    /// This method creates a new Story in the database by inserting the provided Story instance
    /// with an automatically generated ID. It also associates the newly created Story with the
    /// specified Epic by adding its ID to the list of stories in the Epic. It retrieves the current
    /// database state, increments the last item ID, inserts the new Story into the database with
    /// the generated ID, updates the list of stories for the specified Epic, and then writes the
    /// updated state back to the database.
    ///
    /// # Arguments
    ///
    /// * `story` - The Story instance to be created.
    /// * `epic_id` - The ID of the Epic to associate the Story with.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created Story if the operation is successful,
    /// otherwise returns an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use crate::models::{Story, Status};
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// let new_story = Story::new("New Story Name".to_string(), "New Story Description".to_string());
    /// let epic_id = 1; // ID of the associated Epic
    /// match jira_database.create_story(new_story, epic_id) {
    ///     Ok(story_id) => {
    ///         // Handle the ID of the newly created Story
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed_db = self.database.read_db()?;

        let new_id = parsed_db.last_item_id + 1;

        parsed_db.last_item_id = new_id;
        parsed_db.stories.insert(new_id, story);

        parsed_db.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("Could not find epic in the database!".red()))?.stories.push(new_id);

        self.database.write_db(&parsed_db)?;
        Ok(new_id)
    }
    

    /// Deletes an Epic and its associated Stories from the database.
    ///
    /// This method deletes an Epic and its associated Stories from the database by removing
    /// them from the database state. It retrieves the current database state, removes all
    /// Stories associated with the specified Epic, removes the Epic itself, and then writes
    /// the updated state back to the database.
    ///
    /// # Arguments
    ///
    /// * `epic_id` - The ID of the Epic to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success if the operation is successful, otherwise returns
    /// an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// let epic_id = 1; // ID of the Epic to delete
    /// match jira_database.delete_epic(epic_id) {
    ///     Ok(()) => {
    ///         // Handle successful deletion
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut parsed_db = self.database.read_db()?;

        for story_id in &parsed_db.epics.get(&epic_id).ok_or_else(|| anyhow!("Could not find epic in the database!".red()))?.stories {
            parsed_db.stories.remove(story_id);
        }

        parsed_db.epics.remove(&epic_id);

        self.database.write_db(&parsed_db)?;

        Ok(())
    }
    
    /// Deletes a Story from the database.
    ///
    /// This method deletes a Story from the database by removing it from the database state
    /// and removing its association with the specified Epic. It retrieves the current database
    /// state, finds the specified Epic, removes the Story from its list of associated Stories,
    /// removes the Story itself, and then writes the updated state back to the database.
    ///
    /// # Arguments
    ///
    /// * `epic_id` - The ID of the Epic that the Story belongs to.
    /// * `story_id` - The ID of the Story to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success if the operation is successful, otherwise returns
    /// an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// let epic_id = 1; // ID of the Epic that the Story belongs to
    /// let story_id = 1; // ID of the Story to delete
    /// match jira_database.delete_story(epic_id, story_id) {
    ///     Ok(()) => {
    ///         // Handle successful deletion
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn delete_story(&self,epic_id: u32, story_id: u32) -> Result<()> {
        let mut parsed_db = self.database.read_db()?;

        let epic = parsed_db.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("Could not find epic in the database!".red()))?;

        let story_index = epic.stories.iter().position(|id| id == &story_id).ok_or_else(|| anyhow!("Story id not found in epic stories vector".red()))?;

        epic.stories.remove(story_index);

        parsed_db.stories.remove(&story_id);

        self.database.write_db(&parsed_db)?;

        Ok(())
    }
    
    /// Updates the status of an Epic in the database.
    ///
    /// This method updates the status of an Epic in the database to the specified status.
    /// It retrieves the current database state, finds the specified Epic, updates its status,
    /// and then writes the updated state back to the database.
    ///
    /// # Arguments
    ///
    /// * `epic_id` - The ID of the Epic to update.
    /// * `status` - The new status to assign to the Epic.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success if the operation is successful, otherwise returns
    /// an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use crate::models::Status;
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// let epic_id = 1; // ID of the Epic to update
    /// let new_status = Status::InProgress; // New status to assign to the Epic
    /// match jira_database.update_epic_status(epic_id, new_status) {
    ///     Ok(()) => {
    ///         // Handle successful status update
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut parsed_db = self.database.read_db()?;

        parsed_db.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("Could not find epic in the database!".red()))?.status = status;

        self.database.write_db(&parsed_db)?;

        Ok(())
    }
    
    /// Updates the status of a Story in the database.
    ///
    /// This method updates the status of a Story in the database to the specified status.
    /// It retrieves the current database state, finds the specified Story, updates its status,
    /// and then writes the updated state back to the database.
    ///
    /// # Arguments
    ///
    /// * `story_id` - The ID of the Story to update.
    /// * `status` - The new status to assign to the Story.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success if the operation is successful, otherwise returns
    /// an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JiraDatabase;
    /// use crate::models::Status;
    /// use anyhow::Result;
    ///
    /// let jira_database = // instantiate your JiraDatabase instance;
    /// let story_id = 1; // ID of the Story to update
    /// let new_status = Status::InProgress; // New status to assign to the Story
    /// match jira_database.update_story_status(story_id, new_status) {
    ///     Ok(()) => {
    ///         // Handle successful status update
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut parsed_db = self.database.read_db()?;

        parsed_db.stories.get_mut(&story_id).ok_or_else(|| anyhow!("Could not find story in the database!".red()))?.status = status;

        self.database.write_db(&parsed_db)?;
        Ok(())
    }
}

/// JSONFileDatabase represents a database stored in a JSON file.
///
/// This struct stores the file path to the JSON file where the database is stored.
///
/// # Examples
///
/// ```
/// use crate::db::JSONFileDatabase;
///
/// let file_path = "/path/to/database.json".to_string();
/// let json_file_db = JSONFileDatabase { file_path };
/// ```
struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {

    /// Reads the database state from the JSON file.
    ///
    /// This method reads the database state from the JSON file specified by `file_path`.
    /// It reads the file content, deserializes it into a `DBState` struct, and returns it.
    ///
    /// # Errors
    ///
    /// This method can return an error if:
    /// * The file cannot be read.
    /// * The file content cannot be deserialized into a `DBState` struct.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the deserialized `DBState` if the operation is successful,
    /// otherwise returns an `Err` containing an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JSONFileDatabase;
    /// use anyhow::Result;
    ///
    /// let file_path = "/path/to/database.json".to_string();
    /// let json_file_db = JSONFileDatabase { file_path };
    /// match json_file_db.read_db() {
    ///     Ok(db_state) => {
    ///         // Use the retrieved database state
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let parsed_db: DBState = serde_json::from_str(&db_content)?;
        Ok(parsed_db)
    }

    /// Writes the database state to the JSON file.
    ///
    /// This method writes the provided database state to the JSON file specified by `file_path`.
    /// It serializes the `DBState` struct into JSON format and writes it to the file.
    ///
    /// # Arguments
    ///
    /// * `db_state` - A reference to the `DBState` struct containing the database state to be written.
    ///
    /// # Errors
    ///
    /// This method can return an error if:
    /// * The file cannot be written.
    /// * The database state cannot be serialized into JSON format.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::db::JSONFileDatabase;
    /// use crate::models::DBState;
    /// use anyhow::Result;
    ///
    /// let file_path = "/path/to/database.json".to_string();
    /// let json_file_db = JSONFileDatabase { file_path };
    /// let db_state = DBState { /* Populate DBState fields */ };
    /// match json_file_db.write_db(&db_state) {
    ///     Ok(()) => {
    ///         // Database state successfully written to the file
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}

// UNIT TESTING UTILS ------------------------------------------------------------------------------------

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;
    
    pub struct MockDB {
        last_written_state: RefCell<DBState>
    }

    impl MockDB {
        pub fn new() -> Self {
            Self { last_written_state: RefCell::new(DBState { last_item_id: 0, epics: HashMap::new(), stories: HashMap::new() }) }
        }    
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

// ------------------------------------------------------------------------------- UNIT TESTING

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::MockDB;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic.clone());
        
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id), true);
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);
        
        let story_id = result.unwrap();

        let non_existent_epic_id = 999;
        
        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;
        
        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}