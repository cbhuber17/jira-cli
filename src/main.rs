mod models;
mod db;
mod ui;
mod io_utils;
mod navigator;

use std::rc::Rc;
use db::*;
use io_utils::*;
use navigator::*;

/// Main execution loop for the JIRA-like CLI application.
///
/// This function serves as the main entry point for the JIRA-like CLI application.
/// It initializes the database, creates a navigator, and enters a loop where it continuously
/// renders the current page, prompts the user for input, and handles user actions.
/// If an error occurs during page rendering, user input retrieval, or action handling,
/// it displays an error message and waits for the user to press any key to continue.
///
/// // The main entry point of the application
fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));
    
    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", error);
                wait_for_key_press();
            };

            let user_input = get_user_input();

            match page.handle_input(user_input.trim()) {
                Err(error) => {
                    println!("Error getting user input: {}\nPress any key to continue...", error);
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    }
                }         
            }
        } else {
            break;
        }
    }
}
