//! 텍스트 → 검색 토큰. 이후 모든 단계(#10, #11)가 이 위에 쌓인다. (#9)

/// 문서/쿼리 텍스트를 색인 가능한 토큰 목록으로 변환한다.
pub fn tokenize(text: &str) -> Vec<String> {
    todo!("영어/한글 토큰화 — 이슈 #9 참고")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(tokenize(""), Vec::<String>::new());
    }
}
