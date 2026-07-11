use doogle::{Document, Index};
use proptest::prelude::*;

/// 색인 기반 검색이 "이 문서에 쿼리 단어가 포함돼 있다"는 브루트포스 판단과
/// 항상 같은 문서 집합을 찾아내는지 무작위로 대조한다.
///
/// 정확한 순위(score)는 검증하지 않는다 — BM25 순위는 정답이 하나가 아니다.
/// 대신 "찾아야 할 문서를 놓치지 않는가"만 확정적으로 확인한다.
fn word_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z]{2,6}").unwrap()
}

fn document_strategy() -> impl Strategy<Value = Document> {
    (0..20usize, prop::collection::vec(word_strategy(), 3..15)).prop_map(|(id, words)| {
        Document {
            id: id.to_string(),
            title: words.first().cloned().unwrap_or_default(),
            content: words.join(" "),
            url: format!("/{id}.html"),
        }
    })
}

proptest! {
    #[test]
    fn indexed_search_finds_same_documents_as_brute_force(
        docs in prop::collection::vec(document_strategy(), 1..10),
        query in word_strategy(),
    ) {
        let mut idx = Index::new();
        for doc in docs {
            idx.add_document(doc);
        }

        let brute_force = idx.brute_force_matches(&query);
        let indexed: std::collections::HashSet<String> = idx
            .search(&query, usize::MAX)
            .into_iter()
            .map(|r| r.id)
            .collect();

        prop_assert_eq!(indexed, brute_force);
    }
}
