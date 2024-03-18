use ellipse::Ellipse;

/// Generates a formatted string for displaying text in a column with a specified width.
///
/// This function takes a `text` string and a `width` usize as input parameters. It formats
/// the `text` to fit within the specified `width` for column display purposes. If the length
/// of the `text` is equal to the `width`, it returns the original `text`. If the length is less
/// than the `width`, it pads the `text` with spaces on the right to fill the remaining space.
/// If the length exceeds the `width`, it truncates the `text` and adds an ellipsis at the end.
///
/// # Arguments
///
/// * `text` - The input string to be formatted for column display.
/// * `width` - The width of the column in which the text will be displayed.
///
/// # Returns
///
/// A formatted string suitable for displaying within a column of the specified width.
///
/// # Examples
///
/// ```
/// use crate::page_helpers::get_column_string;
///
/// let text = "Example";
/// let width = 10;
/// let formatted_text = get_column_string(text, width);
/// assert_eq!(formatted_text, "Example   ");
/// ```
pub fn get_column_string(text: &str, width: usize) -> String {
    let len = text.len();

    match len.cmp(&width) {
        std::cmp::Ordering::Equal => text.to_owned(),
        std::cmp::Ordering::Less => {
            let left_over = width - len;
            let mut column_string = text.to_owned();

            for _ in 0..left_over {
                column_string.push(' ');
            }

            column_string
        }
        std::cmp::Ordering::Greater => {
            let num_ellepsis = match width {
                0 => "".to_string(),
                1 => ".".to_string(),
                2 => "..".to_string(),
                3 => "...".to_string(),
                _ => "*".to_string()
            };

            if num_ellepsis != "*" {
                return num_ellepsis;
            }

            let result = text.truncate_ellipse(width-3);
            result.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}