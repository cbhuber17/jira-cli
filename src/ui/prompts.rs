use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

/// Contains closures for prompting user input related to Epics and Stories.
///
/// The `Prompts` struct holds closures for various user prompts related to creating, deleting,
/// and updating the status of Epics and Stories. These closures can be invoked to interactively
/// gather input from the user during runtime.
///
/// # Example
///
/// ```
/// use crate::ui::prompts::Prompts;
/// use crate::models::{Epic, Story, Status};
///
/// let prompts = Prompts {
///     create_epic: Box::new(|| Epic::new()),
///     create_story: Box::new(|| Story::new()),
///     delete_epic: Box::new(|| {
///         // Prompt user for confirmation
///         true
///     }),
///     delete_story: Box::new(|| {
///         // Prompt user for confirmation
///         true
///     }),
///     update_status: Box::new(|| {
///         // Prompt user to select a new status
///         Some(Status::InProgress)
///     }),
/// };
/// ```
pub struct Prompts {
    /// Closure for creating a new Epic.
    pub create_epic: Box<dyn Fn() -> Epic>,

    /// Closure for creating a new Story.
    pub create_story: Box<dyn Fn() -> Story>,

    /// Closure for confirming deletion of an Epic.
    pub delete_epic: Box<dyn Fn() -> bool>,

    /// Closure for confirming deletion of a Story.
    pub delete_story: Box<dyn Fn() -> bool>,

    /// Closure for updating the status of an Epic or Story.
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

/// Constructs a new `Prompts` instance.
///
/// This method creates a new `Prompts` instance with default closures for various user prompts
/// related to Epics and Stories. These closures can be invoked to interactively gather input
/// from the user during runtime.
///
/// # Examples
///
/// ```
/// use crate::ui::prompts::Prompts;
///
/// let prompts = Prompts::new();
/// ```
impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

/// Prompts the user to create a new Epic.
///
/// This function displays prompts to the user to input the name and description of a new Epic.
/// It then constructs and returns an `Epic` instance with the provided name and description.
///
/// # Returns
///
/// An `Epic` instance with the provided name and description.
///
/// # Examples
///
/// ```
/// use crate::ui::prompts::create_epic_prompt;
/// use crate::models::Epic;
///
/// let epic = create_epic_prompt();
/// ```
fn create_epic_prompt() -> Epic {
    println!("----------------------------");

    println!("Epic Name: ");

    let epic_name = get_user_input();

    println!("Epic Description: ");

    let epic_desc = get_user_input();

    let epic = Epic::new(epic_name.trim().to_owned(), epic_desc.trim().to_owned());

    epic
}

/// Prompts the user to create a new Story.
///
/// This function displays prompts to the user to input the name and description of a new Story.
/// It then constructs and returns a `Story` instance with the provided name and description.
///
/// # Returns
///
/// A `Story` instance with the provided name and description.
///
/// # Examples
///
/// ```
/// use crate::ui::prompts::create_story_prompt;
/// use crate::models::Story;
///
/// let story = create_story_prompt();
/// ```
fn create_story_prompt() -> Story {
    println!("----------------------------");

    println!("Story Name: ");

    let story_name = get_user_input();

    println!("Story Description: ");

    let story_desc = get_user_input();

    let story = Story::new(story_name.trim().to_owned(), story_desc.trim().to_owned());

    story
}

/// Prompts the user to confirm deletion of an Epic.
///
/// This function displays a prompt to the user to confirm whether they want to delete an Epic.
/// It then reads the user input and returns `true` if the input is "Y" (case insensitive), indicating
/// confirmation for deletion. Otherwise, it returns `false`.
///
/// # Returns
///
/// Returns `true` if the user confirms deletion by entering "Y", otherwise returns `false`.
///
/// # Examples
///
/// ```
/// use crate::ui::prompts::delete_epic_prompt;
///
/// let confirm_deletion = delete_epic_prompt();
/// ```
fn delete_epic_prompt() -> bool {
    println!("----------------------------");

    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]: ");

    let input = get_user_input();

    if input.trim().eq("Y") {
        return true;
    }

    false
}

/// Prompts the user to confirm deletion of a Story.
///
/// This function displays a prompt to the user to confirm whether they want to delete a Story.
/// It then reads the user input and returns `true` if the input is "Y" (case insensitive), indicating
/// confirmation for deletion. Otherwise, it returns `false`.
///
/// # Returns
///
/// Returns `true` if the user confirms deletion by entering "Y", otherwise returns `false`.
///
/// # Examples
///
/// ```
/// use crate::ui::prompts::delete_story_prompt;
///
/// let confirm_deletion = delete_story_prompt();
/// ```
fn delete_story_prompt() -> bool {
    println!("----------------------------");

    println!("Are you sure you want to delete this story? [Y/n]: ");

    let input = get_user_input();

    if input.trim().eq("Y") {
        return true;
    }

    false
}

/// Prompts the user to select a new status for an Epic or a Story.
///
/// This function displays a prompt to the user to select a new status from a list of options.
/// It then reads the user input and returns an `Option<Status>` representing the selected status.
///
/// # Returns
///
/// Returns `Some(Status)` representing the selected status if the user input is a valid status option,
/// otherwise returns `None`.
///
/// # Examples
///
/// ```
/// use crate::ui::prompts::update_status_prompt;
/// use crate::models::Status;
///
/// let new_status = update_status_prompt();
/// ```
fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");

    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED): ");

    let status = get_user_input();

    let status = status.trim().parse::<u8>();

    if let Ok(status) = status {
        match status {
            1 => {
                return Some(Status::Open);
            }
            2 => {
                return Some(Status::InProgress);
            }
            3 => {
                return Some(Status::Resolved);
            }
            4 => {
                return Some(Status::Closed);
            }
            _ => return None
        }
    }

    None
}