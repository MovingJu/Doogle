//! 텍스트 → 검색 토큰. 이후 모든 단계(#10, #11)가 이 위에 쌓인다. (#9)

/// 문서/쿼리 텍스트를 색인 가능한 토큰 목록으로 변환한다.
pub fn tokenize(text: &str) -> Vec<String> {
    todo!("영어/한글 토큰화 — 이슈 #9 참고")
}

const SPLIT_PATTERNS: [char; 5] = [',', ' ', '!', '.', '?'];
/// Split text to Iterator.
///
/// split patterns:
///
/// `,`, ` `, `!`, `.`, `?`
fn split_by_pattern(text: &str) -> impl Iterator<Item = &str> {
    text.split(|ch| SPLIT_PATTERNS.contains(&ch))
}

/// Make bigram of korean words.
///
/// example
/// ```rust
/// use doogle::tokenizer::bigramize_kor;
///
/// assert_eq!(bigramize_kor(&"러스트".chars().collect::<Vec<char>>()), vec!["러스", "스트"]);
/// ```
pub fn bigramize_kor(text: &[char]) -> Vec<String> {
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
}
