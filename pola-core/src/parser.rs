use std::collections::HashMap;

pub fn get_word_count(text: &str) -> HashMap<String, usize> {
    let mut text_string = text.to_lowercase().replace('-', " ");
    text_string.retain(|c| c.is_alphabetic() || c.is_whitespace());

    let mut word_counts = HashMap::new();

    for word in text_string.split_whitespace() {
        let count = word_counts.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    word_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_count() {
        let text = "Java Server Pages (JSP) are a variant of servlets
     • When creating a servlet, we write Java-code, and use Java-
     statements to write HTML into a stream
     • When using JSP we write HTML, and add embedded Java-
     statements";
        let result = get_word_count(text);
        assert_eq!(*result.get("java").expect("java not in Map"), 4);
    }

    #[test]
    fn test_text_with_parantheses() {
        let text = "Hello (World)";
        let result = get_word_count(text);
        assert_eq!(*result.get("world").expect("world not in Map"), 1)
    }

    #[test]
    fn test_text_with_hyphens() {
        let text = "Claude is a well-known LLM. It works so well!";
        let result = get_word_count(text);
        assert_eq!(*result.get("well").expect("well not in Map"), 2)
    }

    #[test]
    fn test_text_with_multiple_punctuations() {
        let text = "Hello (World), I am only a test! Is this going to work? Are we there yet?! 1 + 1 - 1 * 2/1 = 2, is that right?";
        let result = get_word_count(text);
        assert!(!result.contains_key("!"));
        assert!(!result.contains_key("1"));
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = get_word_count(text);
        assert!(result.is_empty())
    }
}
