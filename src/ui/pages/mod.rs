use std::any::Any;
use std::rc::Rc;

use colored::ColoredString;
use itertools::Itertools;
use anyhow::Result;
use anyhow::anyhow;
use colored::Colorize;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

/// Returns a colored string corresponding to the given status.
///
/// # Arguments
///
/// * `status` - A string slice representing the status.
///
/// # Example
///
/// ```
/// use colored::ColoredString;
///
/// let status = "OPEN";
/// let colored_status = get_status_color(status);
/// println!("{}", colored_status);
/// ```
///
/// # Panics
///
/// If the provided `status` is not recognized, the function returns an empty colored string.
///
/// # Notes
///
/// - The recognized status values are "OPEN", "IN PROGRESS", "RESOLVED", and "CLOSED".
///
/// # Returns
///
/// A `ColoredString` representing the colorized version of the status.
///
/// # Dependencies
///
/// This function requires the `colored` crate to be included in your project.
///
fn get_status_color(status: &str) -> ColoredString {
    match status.trim() {
        "OPEN" => "OPEN".purple(),
        "IN PROGRESS" => "IN PROGRESS".yellow(),
        "RESOLVED" => "RESOLVED".green(),
        "CLOSED" => "CLOSED".blue(),
        _ => "".clear()
    }
}

/// A trait representing a page in the user interface.
///
/// Pages in the user interface typically have two main responsibilities: drawing
/// the contents of the page and handling user input.
///
/// # Example
///
/// ```
/// use crate::ui::pages::Page;
/// use crate::ui::actions::Action;
///
/// struct HomePage;
///
/// impl Page for HomePage {
///     fn draw_page(&self) -> Result<(), Box<dyn std::error::Error>> {
///         // Draw the contents of the home page
///         Ok(())
///     }
///
///     fn handle_input(&self, input: &str) -> Result<Option<Action>, Box<dyn std::error::Error>> {
///         // Handle user input on the home page
///         Ok(None)
///     }
///
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
/// }
/// ```
pub trait Page {
    /// Draws the contents of the page.
    ///
    /// This method is responsible for rendering the contents of the page to
    /// the user interface.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure in drawing the page.
    fn draw_page(&self) -> Result<()>;

    /// Handles user input on the page.
    ///
    /// This method is responsible for processing user input and triggering
    /// appropriate actions or state changes.
    ///
    /// # Arguments
    ///
    /// * `input` - The user input to be handled.
    ///
    /// # Returns
    ///
    /// A `Result` containing an optional `Action` to be executed based on the input,
    /// or an error if input handling fails.
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;

    /// Returns a reference to the trait object as `dyn Any`.
    ///
    /// This method is used for downcasting a trait object to a concrete type.
    ///
    /// # Returns
    ///
    /// A reference to the trait object as `dyn Any`.
    fn as_any(&self) -> &dyn Any;
}

/// Represents the home page of the user interface.
///
/// The home page typically serves as the main entry point of the application's user interface,
/// providing an overview or navigation options.
///
/// # Example
///
/// ```
/// use crate::ui::pages::HomePage;
/// use crate::JiraDatabase;
/// use std::rc::Rc;
///
/// let database = Rc::new(JiraDatabase::new());
/// let home_page = HomePage { db: database.clone() };
/// ```
pub struct HomePage {

    /// Reference-counted pointer to the JIRA database.
    ///
    /// This field holds a shared reference to the JIRA database, allowing the home page to access
    /// and interact with the underlying data.
    pub db: Rc<JiraDatabase>
}

impl Page for HomePage {

    /// Draws the contents of the home page.
    ///
    /// This method prints the list of epics from the JIRA database, displaying their IDs, names,
    /// and statuses in a formatted table on the command-line interface (CLI).
    ///
    /// # Errors
    ///
    /// Returns an error if there are issues reading the JIRA database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::HomePage;
    /// use crate::JiraDatabase;
    /// use std::rc::Rc;
    ///
    /// let database = Rc::new(JiraDatabase::new());
    /// let home_page = HomePage { db: database.clone() };
    ///
    /// // Assuming database has been populated with epics
    /// let result = home_page.draw_page();
    /// assert!(result.is_ok());
    /// ```
    fn draw_page(&self) -> Result<()> {
        println!("{}", "----------------------------- EPICS -----------------------------".cyan());
        println!("{}", "     id     |               name               |      status      ".cyan());

        let epics = self.db.read_db()?.epics;

        for id in epics.keys().sorted() {
            let epic = &epics[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&epic.name, 32);
            let status_col = get_column_string(&epic.status.to_string(), 17);
            let status_color = get_status_color(&status_col);

            println!("{} {} {} {} {}",
                                    id_col,
                                    "|".cyan(),
                                    name_col,
                                    "|".cyan(),
                                    status_color);
        }

        println!();
        println!();

        println!("{} | {} | {}", "[q] quit".red(),
                                 "[c] create epic".green(),
                                 "[:id:] navigate to epic".yellow());

        Ok(())
    }

    /// Handles user input on the home page.
    ///
    /// This method interprets the user input and maps it to corresponding actions. If the input
    /// matches predefined commands such as quitting the application or creating a new epic, it
    /// returns the corresponding action. If the input represents an epic ID, it checks if the
    /// ID exists in the JIRA database and returns an action to navigate to the details of that epic.
    /// If the input does not match any predefined command or epic ID, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `input` - The user input to be handled.
    ///
    /// # Errors
    ///
    /// Returns an error if there are issues reading the JIRA database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::HomePage;
    /// use crate::JiraDatabase;
    /// use crate::ui::actions::Action;
    /// use std::rc::Rc;
    ///
    /// let database = Rc::new(JiraDatabase::new());
    /// let home_page = HomePage { db: database.clone() };
    ///
    /// // Assuming database has been populated with epics
    /// let result = home_page.handle_input("1");
    /// assert!(result.is_ok());
    /// let action = result.unwrap();
    /// assert_eq!(action, Some(Action::NavigateToEpicDetail { epic_id: 1 }));
    /// ```
    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epics = self.db.read_db()?.epics;

        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            input => {
                if let Ok(epic_id) = input.parse::<u32>() {
                    if epics.contains_key(&epic_id) {
                        return Ok(Some(Action::NavigateToEpicDetail { epic_id }));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Returns a reference to the trait object as `dyn Any`.
    ///
    /// This method is used for downcasting a trait object to a concrete type.
    ///
    /// # Returns
    ///
    /// A reference to the trait object as `dyn Any`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::HomePage;
    /// use std::any::Any;
    ///
    /// let home_page = HomePage { /* initialize HomePage instance */ };
    /// let trait_object_ref = home_page.as_any();
    ///
    /// // Downcast trait object to concrete type
    /// if let Some(downcasted_home_page) = trait_object_ref.downcast_ref::<HomePage>() {
    ///     // Access methods and fields specific to HomePage
    /// } else {
    ///     // Trait object is not of type HomePage
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents the detail page for an Epic in the user interface.
///
/// The EpicDetail page provides detailed information about a specific Epic,
/// including its ID and associated data from the JIRA database.
///
/// # Example
///
/// ```
/// use crate::ui::pages::EpicDetail;
/// use crate::JiraDatabase;
/// use std::rc::Rc;
///
/// let database = Rc::new(JiraDatabase::new());
/// let epic_detail_page = EpicDetail { epic_id: 1, db: database.clone() };
/// ```
pub struct EpicDetail {
    /// The ID of the Epic being displayed.
    ///
    /// This field holds the unique identifier of the Epic for which detailed
    /// information is being displayed.
    pub epic_id: u32,

    /// Reference-counted pointer to the JIRA database.
    ///
    /// This field holds a shared reference to the JIRA database, allowing the
    /// EpicDetail page to access and display data associated with the specified Epic.
    pub db: Rc<JiraDatabase>
}

impl Page for EpicDetail {

    /// Draws the contents of the EpicDetail page.
    ///
    /// This method prints detailed information about the Epic, including its ID, name, description,
    /// status, and associated stories. It retrieves the relevant data from the JIRA database and
    /// formats it into a structured output on the command-line interface (CLI).
    ///
    /// # Errors
    ///
    /// Returns an error if the Epic with the specified ID is not found in the JIRA database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::EpicDetail;
    /// use crate::JiraDatabase;
    /// use std::rc::Rc;
    ///
    /// let database = Rc::new(JiraDatabase::new());
    /// let epic_detail_page = EpicDetail { epic_id: 1, db: database.clone() };
    ///
    /// // Assuming database has been populated with the specified Epic and its associated stories
    /// let result = epic_detail_page.draw_page();
    /// assert!(result.is_ok());
    /// ```
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("could not find epic!".red().bold()))?;

        println!("{}", "------------------------------ EPIC ------------------------------".cyan());
        println!("{}", "  id  |     name     |         description         |    status    ".cyan());

        let id_col = get_column_string(&self.epic_id.to_string(), 5);
        let name_col = get_column_string(&epic.name, 12);
        let desc_col = get_column_string(&epic.description, 27);
        let status_col = get_column_string(&epic.status.to_string(), 13);
        let status_color = get_status_color(&status_col);

        println!("{} {} {} {} {} {} {}",
                                     id_col,
                                     "|".cyan(),
                                     name_col,
                                     "|".cyan(),
                                     desc_col,
                                     "|".cyan(),
                                     status_color);
        
        println!();

        println!("{}", "---------------------------- STORIES ----------------------------".cyan());
        println!("{}", "     id     |               name               |      status      ".cyan());

        let stories = &db_state.stories;

        for id in epic.stories.iter().sorted() {
            let story = &stories[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&story.name, 32);
            let status_col = get_column_string(&story.status.to_string(), 17);
            let status_color = get_status_color(&status_col);

            println!("{} {} {} {} {}",
                                   id_col,
                                   "|".cyan(),
                                   name_col,
                                   "|".cyan(),
                                   status_color);
        }

        println!();
        println!();

        println!("{} {} {} {} {} {} {} {} {}",
                                            "[p] previous".green(),
                                            "|".cyan(),
                                            "[u] update epic".yellow(),
                                            "|".cyan(),
                                            "[d] delete epic".red(),
                                            "|".cyan(),
                                            "[c] create story".blue(),
                                            "|".cyan(),
                                            "[:id:] navigate to story".purple());

        Ok(())
    }

    /// Handles user input on the EpicDetail page.
    ///
    /// This method interprets the user input and maps it to corresponding actions. If the input
    /// matches predefined commands such as navigating to the previous page, updating the epic status,
    /// deleting the epic, or creating a new story, it returns the corresponding action. If the input
    /// represents a story ID associated with the epic, it checks if the ID exists in the JIRA database
    /// and returns an action to navigate to the details of that story. If the input does not match any
    /// predefined command or story ID, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `input` - The user input to be handled.
    ///
    /// # Errors
    ///
    /// Returns an error if there are issues reading the JIRA database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::EpicDetail;
    /// use crate::JiraDatabase;
    /// use crate::ui::actions::Action;
    /// use std::rc::Rc;
    ///
    /// let database = Rc::new(JiraDatabase::new());
    /// let epic_detail_page = EpicDetail { epic_id: 1, db: database.clone() };
    ///
    /// // Assuming database has been populated with stories
    /// let result = epic_detail_page.handle_input("1");
    /// assert!(result.is_ok());
    /// let action = result.unwrap();
    /// assert_eq!(action, Some(Action::NavigateToStoryDetail { epic_id: 1, story_id: 1 }));
    /// ```
    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let db_state = self.db.read_db()?;

        let stories = db_state.stories;

        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus { epic_id: self.epic_id })),
            "d" => Ok(Some(Action::DeleteEpic { epic_id: self.epic_id })),
            "c" => Ok(Some(Action::CreateStory { epic_id: self.epic_id })),
            input => {
                if let Ok(story_id) = input.parse::<u32>() {
                    if stories.contains_key(&story_id) {
                        return Ok(Some(Action::NavigateToStoryDetail { epic_id: self.epic_id, story_id }));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Returns a reference to the trait object as `dyn Any`.
    ///
    /// This method is used for downcasting a trait object to a concrete type.
    ///
    /// # Returns
    ///
    /// A reference to the trait object as `dyn Any`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::EpicDetail;
    /// use std::any::Any;
    ///
    /// let epic_detail_page = EpicDetail { /* initialize EpicDetail instance */ };
    /// let trait_object_ref = epic_detail_page.as_any();
    ///
    /// // Downcast trait object to concrete type
    /// if let Some(downcasted_epic_detail_page) = trait_object_ref.downcast_ref::<EpicDetail>() {
    ///     // Access methods and fields specific to EpicDetail
    /// } else {
    ///     // Trait object is not of type EpicDetail
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents the detail page for a Story in the user interface.
///
/// The StoryDetail page provides detailed information about a specific Story
/// within an Epic, including its ID, associated Epic ID, and relevant data
/// from the JIRA database.
///
/// # Example
///
/// ```
/// use crate::ui::pages::StoryDetail;
/// use crate::JiraDatabase;
/// use std::rc::Rc;
///
/// let database = Rc::new(JiraDatabase::new());
/// let story_detail_page = StoryDetail { epic_id: 1, story_id: 1, db: database.clone() };
/// ```
pub struct StoryDetail {

    /// The ID of the Epic to which the Story belongs.
    ///
    /// This field holds the unique identifier of the Epic associated with
    /// the Story for which detailed information is being displayed.
    pub epic_id: u32,

    /// The ID of the Story being displayed.
    ///
    /// This field holds the unique identifier of the Story for which detailed
    /// information is being displayed.
    pub story_id: u32,

    /// Reference-counted pointer to the JIRA database.
    ///
    /// This field holds a shared reference to the JIRA database, allowing the
    /// StoryDetail page to access and display data associated with the specified Story.
    pub db: Rc<JiraDatabase>
}

impl Page for StoryDetail {

    /// Draws the contents of the StoryDetail page.
    ///
    /// This method prints detailed information about the Story, including its ID, name,
    /// description, and status. It retrieves the relevant data from the JIRA database
    /// and formats it into a structured output on the command-line interface (CLI).
    ///
    /// # Errors
    ///
    /// Returns an error if the Story with the specified ID is not found in the JIRA database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::StoryDetail;
    /// use crate::JiraDatabase;
    /// use std::rc::Rc;
    ///
    /// let database = Rc::new(JiraDatabase::new());
    /// let story_detail_page = StoryDetail { epic_id: 1, story_id: 1, db: database.clone() };
    ///
    /// // Assuming database has been populated with the specified Story
    /// let result = story_detail_page.draw_page();
    /// assert!(result.is_ok());
    /// ```
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let story = db_state.stories.get(&self.story_id).ok_or_else(|| anyhow!("could not find story!".red().bold()))?;

        println!("{}", "------------------------------ STORY ------------------------------".cyan());
        println!("{}", "  id  |     name     |         description         |    status     ".cyan());
        
        let id_col = get_column_string(&self.story_id.to_string(), 5);
        let name_col = get_column_string(&story.name, 12);
        let desc_col = get_column_string(&story.description, 27);
        let status_col = get_column_string(&story.status.to_string(), 13);
        let status_color = get_status_color(&status_col);

        println!("{} {} {} {} {} {} {}",
                                     id_col,
                                     "|".cyan(),
                                     name_col,
                                     "|".cyan(),
                                     desc_col,
                                     "|".cyan(),
                                     status_color);

        println!();
        println!();

        println!("{} {} {} {} {}", "[p] previous".green(),
                                   "|".cyan(),
                                   "[u] update story".yellow(), 
                                   "|".cyan(),
                                   "[d] delete story".red());

        Ok(())
    }

    /// Handles user input on the StoryDetail page.
    ///
    /// This method interprets the user input and maps it to corresponding actions. If the input
    /// matches predefined commands such as navigating to the previous page, updating the story status,
    /// or deleting the story, it returns the corresponding action. If the input does not match any
    /// predefined command, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `input` - The user input to be handled.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ui::pages::StoryDetail;
    /// use crate::ui::actions::Action;
    ///
    /// let story_detail_page = StoryDetail { /* initialize StoryDetail instance */ };
    ///
    /// let result = story_detail_page.handle_input("p");
    /// assert!(result.is_ok());
    /// let action = result.unwrap();
    /// assert_eq!(action, Some(Action::NavigateToPreviousPage));
    /// ```
    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus { story_id: self.story_id })),
            "d" => Ok(Some(Action::DeleteStory { epic_id: self.epic_id, story_id: self.story_id })),
            _ => Ok(None)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// -------------------------------------------------------------- UNIT TESTING

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::test_utils::MockDB};
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert_eq!(page.draw_page().is_ok(), true);
        }
        
        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(page.handle_input(&valid_epic_id).unwrap(), Some(Action::NavigateToEpicDetail { epic_id: 1 }));
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = EpicDetail { epic_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateEpicStatus { epic_id: 1 }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteEpic { epic_id: 1 }));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateStory { epic_id: 1 }));
            assert_eq!(page.handle_input(&story_id.to_string()).unwrap(), Some(Action::NavigateToStoryDetail { epic_id: 1, story_id: 2 }));
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }

    mod story_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let _ = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateStoryStatus { story_id }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteStory { epic_id, story_id }));
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }
}