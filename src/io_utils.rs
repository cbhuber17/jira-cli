use std::io;

/// Reads user input from the standard input (stdin) and returns it as a string.
///
/// This function reads user input from the standard input (stdin) and returns it as a string.
/// It prompts the user to input data, reads the input from the standard input, and returns
/// the entered text as a string. If an error occurs during input reading, this function
/// will panic.
///
/// # Returns
///
/// Returns a string containing the user input.
///
/// # Panics
///
/// This function will panic if an error occurs during input reading from the standard input.
///
/// # Examples
///
/// ```
/// use crate::io_utils::get_user_input;
///
/// let user_input = get_user_input();
/// println!("User input: {}", user_input);
/// ```
pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

/// Waits for a key press from the user.
///
/// This function waits for a key press from the user by reading a line from the standard input (stdin).
/// It prompts the user to press a key and waits until a key is pressed. This function is useful for
/// pausing execution until the user interacts with the program.
///
/// # Panics
///
/// This function will panic if an error occurs during input reading from the standard input.
///
/// # Examples
///
/// ```
/// use crate::io_utils::wait_for_key_press;
///
/// println!("Press any key to continue...");
/// wait_for_key_press();
/// ```
pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}