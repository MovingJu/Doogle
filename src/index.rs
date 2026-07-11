use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tokenizer::tokenize;

/// 색인 대상 문서. 어떤 사이트든 이 형태로만 변환해서 넘기면 색인 가능하다.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub url: String,
}

/// 검색 결과 하나.
#[derive(Clone, Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub score: f64,
}

const BM25_K1: f64 = 1.5;
const BM25_B: f64 = 0.75;
/// 제목에 나온 단어는 본문보다 중요하다고 보고, 색인할 때 제목 텍스트를 이만큼 반복해 가중치를 준다.
const TITLE_BOOST: usize = 3;

/// 역색인(inverted index) 기반 인메모리 검색 엔진.
#[derive(Default)]
pub struct Index {
    documents: HashMap<String, Document>,
    /// term -> (doc_id -> 그 문서 안에서의 등장 횟수)
    postings: HashMap<String, HashMap<String, u32>>,
    doc_lengths: HashMap<String, usize>,
    total_length: usize,
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// 문서를 색인에 추가한다. 같은 id로 다시 추가하면 이전 내용을 덮어쓴다.
    pub fn add_document(&mut self, doc: Document) {
        self.remove_document(&doc.id);

        let boosted_title = doc.title.repeat(TITLE_BOOST);
        let text = format!("{boosted_title} {}", doc.content);
        let tokens = tokenize(&text);

        self.total_length += tokens.len();
        self.doc_lengths.insert(doc.id.clone(), tokens.len());

        let mut freq: HashMap<String, u32> = HashMap::new();
        for t in tokens {
            *freq.entry(t).or_insert(0) += 1;
        }
        for (term, count) in freq {
            self.postings
                .entry(term)
                .or_default()
                .insert(doc.id.clone(), count);
        }

        self.documents.insert(doc.id.clone(), doc);
    }

    /// 기존 문서를 색인에서 완전히 제거한다 (재색인 시 내부적으로 사용).
    fn remove_document(&mut self, id: &str) {
        if let Some(len) = self.doc_lengths.remove(id) {
            self.total_length -= len;
        }
        self.documents.remove(id);
        self.postings.retain(|_, docs| {
            docs.remove(id);
            !docs.is_empty()
        });
    }

    fn avg_doc_length(&self) -> f64 {
        if self.documents.is_empty() {
            0.0
        } else {
            self.total_length as f64 / self.documents.len() as f64
        }
    }

    /// BM25로 점수를 매겨 상위 `limit`개 결과를 반환한다.
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let query_terms = tokenize(query);
        if query_terms.is_empty() {
            return Vec::new();
        }

        let n = self.documents.len() as f64;
        let avgdl = self.avg_doc_length().max(1.0);
        let mut scores: HashMap<String, f64> = HashMap::new();

        for term in &query_terms {
            let Some(postings) = self.postings.get(term) else {
                continue;
            };
            let df = postings.len() as f64;
            let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();

            for (doc_id, &tf) in postings {
                let dl = *self.doc_lengths.get(doc_id).unwrap_or(&0) as f64;
                let tf = tf as f64;
                let denom = tf + BM25_K1 * (1.0 - BM25_B + BM25_B * dl / avgdl);
                let score = idf * (tf * (BM25_K1 + 1.0)) / denom;
                *scores.entry(doc_id.clone()).or_insert(0.0) += score;
            }
        }

        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .filter(|(_, score)| *score > 0.0)
            .map(|(doc_id, score)| {
                let doc = &self.documents[&doc_id];
                SearchResult {
                    id: doc.id.clone(),
                    title: doc.title.clone(),
                    url: doc.url.clone(),
                    snippet: make_snippet(&doc.content, &query_terms),
                    score,
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(limit);
        results
    }

    /// 브루트포스 선형 스캔 — 색인 없이 모든 문서를 훑어서 쿼리 단어가 하나라도
    /// 포함된 문서 id 집합을 반환한다. 색인 기반 검색이 "찾아야 할 문서"를
    /// 놓치지 않는지 오라클 테스트로 대조할 때만 쓴다 (운영 경로에선 안 씀).
    pub fn brute_force_matches(&self, query: &str) -> std::collections::HashSet<String> {
        let query_terms = tokenize(query);
        self.documents
            .values()
            .filter(|doc| {
                let text = format!("{} {}", doc.title, doc.content);
                let doc_tokens = tokenize(&text);
                query_terms.iter().any(|q| doc_tokens.contains(q))
            })
            .map(|doc| doc.id.clone())
            .collect()
    }
}

/// 쿼리 단어가 처음 등장하는 위치 주변을 잘라내 미리보기 문자열을 만든다.
/// 바이트 인덱스로 자르면 한글 UTF-8 경계에서 패닉날 수 있어 char 단위로만 다룬다.
fn make_snippet(content: &str, query_terms: &[String]) -> String {
    let chars: Vec<char> = content.chars().collect();
    let lower_chars: Vec<char> = content.to_lowercase().chars().collect();

    let mut best_pos = None;
    for term in query_terms {
        let term_chars: Vec<char> = term.chars().collect();
        if term_chars.is_empty() {
            continue;
        }
        if let Some(pos) = find_char_subsequence(&lower_chars, &term_chars) {
            best_pos = Some(best_pos.map_or(pos, |b: usize| b.min(pos)));
        }
    }

    let center = best_pos.unwrap_or(0);
    let start = center.saturating_sub(40);
    let end = (center + 120).min(chars.len());
    let snippet: String = chars[start..end].iter().collect();
    if start > 0 {
        format!("...{snippet}")
    } else {
        snippet
    }
}

fn find_char_subsequence(haystack: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(id: &str, title: &str, content: &str) -> Document {
        Document {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            url: format!("/{id}.html"),
        }
    }

    #[test]
    fn empty_index_returns_no_results() {
        let idx = Index::new();
        assert!(idx.search("rust", 10).is_empty());
    }

    #[test]
    fn finds_document_containing_query_word() {
        let mut idx = Index::new();
        idx.add_document(doc("1", "Rust 소유권", "Rust는 소유권 모델로 메모리를 관리한다"));
        idx.add_document(doc("2", "고양이 키우기", "고양이는 하루에 15시간을 잔다"));

        let results = idx.search("소유권", 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "1");
    }

    #[test]
    fn document_with_more_term_occurrences_ranks_higher() {
        let mut idx = Index::new();
        idx.add_document(doc(
            "low",
            "짧은 글",
            "러스트 이야기가 한 번 나온다",
        ));
        idx.add_document(doc(
            "high",
            "러스트 러스트 러스트",
            "러스트, 러스트, 온통 러스트 이야기뿐이다 러스트",
        ));

        let results = idx.search("러스트", 10);
        assert_eq!(results[0].id, "high");
    }

    #[test]
    fn query_with_no_matches_returns_empty() {
        let mut idx = Index::new();
        idx.add_document(doc("1", "고양이", "고양이는 귀엽다"));
        assert!(idx.search("transformer", 10).is_empty());
    }

    #[test]
    fn limit_truncates_results() {
        let mut idx = Index::new();
        for i in 0..5 {
            idx.add_document(doc(&i.to_string(), "러스트 문서", "러스트 내용"));
        }
        let results = idx.search("러스트", 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn re_adding_same_id_replaces_old_content() {
        let mut idx = Index::new();
        idx.add_document(doc("1", "옛 제목", "옛 내용 자바스크립트"));
        idx.add_document(doc("1", "새 제목", "새 내용 러스트"));

        assert_eq!(idx.document_count(), 1);
        assert!(idx.search("자바스크립트", 10).is_empty());
        assert_eq!(idx.search("러스트", 10).len(), 1);
    }

    #[test]
    fn snippet_does_not_panic_on_korean_utf8_boundaries() {
        let mut idx = Index::new();
        // 멀티바이트 문자만 200개 반복 — 바이트 인덱스로 슬라이싱했다면 문자 경계를
        // 벗어나 패닉났을 크기. "가나다라" 반복은 바이그램 "가나"를 실제로 색인에 남긴다.
        idx.add_document(doc("1", "한글 테스트", &"가나다라".repeat(50)));
        let results = idx.search("가나", 10);
        assert_eq!(results.len(), 1);
        assert!(!results[0].snippet.is_empty());
    }
}
