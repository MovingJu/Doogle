/// 한글 완성형 음절 범위 (U+AC00 ~ U+D7A3)인지 확인.
fn is_hangul(c: char) -> bool {
    ('\u{AC00}'..='\u{D7A3}').contains(&c)
}

/// 텍스트를 검색 가능한 토큰으로 분해한다.
///
/// 영어/숫자는 단어 단위로 소문자화해서 하나의 토큰으로 만든다.
/// 한글은 형태소 분석기 없이도 부분 문자열 검색이 되도록, 단어 전체 토큰에
/// 더해 인접한 두 글자씩 묶은 바이그램도 함께 색인한다.
pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for c in text.chars() {
        if c.is_alphanumeric() {
            current.push(c);
        } else if !current.is_empty() {
            push_token(&current, &mut tokens);
            current.clear();
        }
    }
    if !current.is_empty() {
        push_token(&current, &mut tokens);
    }
    tokens
}

fn push_token(word: &str, tokens: &mut Vec<String>) {
    let lower = word.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let has_hangul = chars.iter().any(|&c| is_hangul(c));

    if has_hangul && chars.len() >= 2 {
        for w in chars.windows(2) {
            tokens.push(w.iter().collect());
        }
        // 두 글자 단어는 바이그램이 이미 전체 단어와 같으므로 중복 추가하지 않음
        if chars.len() > 2 {
            tokens.push(lower);
        }
    } else {
        tokens.push(lower);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_english_words_and_lowercases() {
        assert_eq!(tokenize("Hello World"), vec!["hello", "world"]);
    }

    #[test]
    fn strips_punctuation() {
        assert_eq!(tokenize("rust, cargo! toml?"), vec!["rust", "cargo", "toml"]);
    }

    #[test]
    fn empty_input_yields_no_tokens() {
        assert_eq!(tokenize(""), Vec::<String>::new());
        assert_eq!(tokenize("   ,.!  "), Vec::<String>::new());
    }

    #[test]
    fn korean_word_gets_bigrams_plus_whole_word() {
        // "러스트" (3글자) -> 바이그램 "러스", "스트" + 전체 단어 "러스트"
        let tokens = tokenize("러스트");
        assert_eq!(tokens, vec!["러스", "스트", "러스트"]);
    }

    #[test]
    fn single_korean_syllable_has_no_bigram() {
        // 한 글자짜리는 바이그램을 만들 수 없으므로 단어 자체만 토큰이 됨
        assert_eq!(tokenize("책"), vec!["책"]);
    }

    #[test]
    fn mixed_korean_and_english() {
        let tokens = tokenize("Rust 언어");
        assert_eq!(tokens, vec!["rust", "언어"]);
    }
}
