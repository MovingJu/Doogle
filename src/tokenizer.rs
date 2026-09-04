//! Text -> search tokens. Every later stage (#10, #11) builds on this. (#9)

/// Converts document/query text into a list of indexable tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    let words = split_by_pattern(text);
    let mut tokens: Vec<String> = Vec::new();
    for word in words {
        if word.is_ascii() {
            tokens.push(word.to_lowercase());
        } else {
            let mut token_bigrams = bigramize_kor(word.chars().collect::<Vec<char>>());
            tokens.append(&mut token_bigrams);
        }
    }
    tokens
}

const SPLIT_PATTERNS: [char; 5] = [',', ' ', '!', '.', '?'];
/// Split text to Iterator.
///
/// split patterns:
///
/// `,`, ` `, `!`, `.`, `?`
fn split_by_pattern(text: &str) -> impl Iterator<Item = &str> {
    text.split(|ch| SPLIT_PATTERNS.contains(&ch))
        .filter(|&text| !text.is_empty())
}

/// Make bigram of korean words.
///
/// example
/// ```rust
/// use doogle::tokenizer::bigramize_kor;
///
/// assert_eq!(bigramize_kor("러스트".chars().collect::<Vec<char>>()), vec!["러스", "스트"]);
/// ```
pub fn bigramize_kor(text: Vec<char>) -> Vec<String> {
    text.windows(2)
        .map(|slice| {
            slice.iter().fold(String::new(), |mut acc, elem| {
                acc.push(*elem);
                acc
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(tokenize(""), Vec::<String>::new());
    }

    #[test]
    fn ascii_input() {
        assert_eq!(tokenize("english, 123!"), vec!["english", "123"]);
    }

    #[test]
    fn non_ascii_input() {
        assert_eq!(tokenize("러스트"), vec!["러스", "스트"]);
    }

    #[test]
    fn mixed_input() {
        assert_eq!(tokenize("eng, 안녕?"), vec!["eng", "안녕"]);
    }
}
