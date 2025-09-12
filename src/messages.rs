use crate::config::Language;

/// Returns a greeting message for the specified language.
///
/// # Examples
/// ```
/// use telegram_bot_template::{messages, Language};
/// assert_eq!(messages::greeting(Language::En), "Hello!");
/// ```
pub fn greeting(language: Language) -> &'static str {
    match language {
        Language::En => "Hello!",
        Language::Ru => "Привет!",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_in_english() {
        assert_eq!(greeting(Language::En), "Hello!");
    }

    #[test]
    fn greet_in_russian() {
        assert_eq!(greeting(Language::Ru), "Привет!");
    }
}
