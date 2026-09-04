//! Inverted index + ranking. Filled in by #10 (build) and #11 (scoring).

use crate::tokenizer::tokenize;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Document {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub url: String,
}

pub struct Posting {
    pub doc_id: u32,
    pub term_freq: u32,
}

pub struct ScoredDoc {
    pub doc_id: u32,
    pub score: f64,
}

pub struct Index {
    pub postings: HashMap<String, Vec<Posting>>,
    pub documents: HashMap<u32, Document>,
}

/// Tokenizes a document's title+content and counts occurrences of each token.
fn term_frequencies(doc: &Document) -> HashMap<String, u32> {
    let text = format!("{} {}", doc.title, doc.content);
    let mut term_freq_map: HashMap<String, u32> = HashMap::new();
    for token in tokenize(&text) {
        *term_freq_map.entry(token).or_insert(0) += 1;
    }
    term_freq_map
}

/// Folds one document's term-frequency map into the shared postings map,
/// appending a single `Posting` per token (never one per occurrence).
fn merge_postings(
    postings: &mut HashMap<String, Vec<Posting>>,
    doc_id: u32,
    term_freqs: HashMap<String, u32>,
) {
    for (token, freq) in term_freqs {
        postings.entry(token).or_default().push(Posting {
            doc_id,
            term_freq: freq,
        });
    }
}

impl Index {
    /// Builds an inverted index from a list of documents. (#10)
    ///
    /// Tokenization is CPU-bound and independent per document, so it runs in
    /// parallel across threads via rayon; only the (cheap) merge into the
    /// shared postings map happens sequentially afterwards.
    pub fn build(docs: Vec<Document>) -> Index {
        let per_doc_term_freqs: Vec<(u32, HashMap<String, u32>)> = docs
            .par_iter()
            .map(|doc| (doc.id, term_frequencies(doc)))
            .collect();

        let mut postings: HashMap<String, Vec<Posting>> = HashMap::new();
        for (doc_id, term_freqs) in per_doc_term_freqs {
            merge_postings(&mut postings, doc_id, term_freqs);
        }

        let documents: HashMap<u32, Document> = docs.into_iter().map(|doc| (doc.id, doc)).collect();

        Index {
            postings,
            documents,
        }
    }

    /// Returns documents sorted by relevance to the query. (#11)
    pub fn score(&self, _query: &str) -> Vec<ScoredDoc> {
        todo!("BM25 scoring — see issue #11")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(id: u32, title: &str, content: &str) -> Document {
        Document {
            id,
            title: title.into(),
            content: content.into(),
            url: format!("/{id}"),
        }
    }

    #[test]
    fn term_frequencies_counts_repeated_token() {
        let d = doc(1, "cat", "cat cat dog");
        let freqs = term_frequencies(&d);
        assert_eq!(freqs.get("cat"), Some(&3));
        assert_eq!(freqs.get("dog"), Some(&1));
    }

    #[test]
    fn term_frequencies_covers_title_and_content() {
        let d = doc(1, "Rust 소개", "러스트는 빠르다");
        let freqs = term_frequencies(&d);
        assert!(freqs.contains_key("rust"));
        assert!(freqs.contains_key("러스"));
    }

    #[test]
    fn merge_postings_appends_one_posting_per_token() {
        let mut postings: HashMap<String, Vec<Posting>> = HashMap::new();
        let mut freqs = HashMap::new();
        freqs.insert("cat".to_string(), 3);

        merge_postings(&mut postings, 1, freqs);

        let cat_postings = postings.get("cat").unwrap();
        assert_eq!(cat_postings.len(), 1);
        assert_eq!(cat_postings[0].doc_id, 1);
        assert_eq!(cat_postings[0].term_freq, 3);
    }

    #[test]
    fn merge_postings_accumulates_across_documents() {
        let mut postings: HashMap<String, Vec<Posting>> = HashMap::new();

        let mut freqs1 = HashMap::new();
        freqs1.insert("cat".to_string(), 1);
        merge_postings(&mut postings, 1, freqs1);

        let mut freqs2 = HashMap::new();
        freqs2.insert("cat".to_string(), 2);
        merge_postings(&mut postings, 2, freqs2);

        let cat_postings = postings.get("cat").unwrap();
        assert_eq!(cat_postings.len(), 2);
        assert!(
            cat_postings
                .iter()
                .any(|p| p.doc_id == 1 && p.term_freq == 1)
        );
        assert!(
            cat_postings
                .iter()
                .any(|p| p.doc_id == 2 && p.term_freq == 2)
        );
    }

    #[test]
    fn test_build_and_lookup() {
        let docs = vec![
            doc(1, "Rust 소개", "러스트는 빠르다"),
            doc(2, "Python 소개", "파이썬은 쉽다"),
        ];
        let index = Index::build(docs);

        // "러스" 토큰은 문서 1에만 있어야 함
        let postings = index.postings.get("러스").unwrap();
        assert_eq!(postings.len(), 1);
        assert_eq!(postings[0].doc_id, 1);
    }

    #[test]
    fn build_keeps_term_freq_within_single_document() {
        let docs = vec![doc(1, "cat", "cat cat dog")];
        let index = Index::build(docs);

        let cat_postings = index.postings.get("cat").unwrap();
        assert_eq!(cat_postings.len(), 1);
        assert_eq!(cat_postings[0].term_freq, 3);
    }

    #[test]
    fn build_stores_documents_for_lookup() {
        let docs = vec![doc(1, "Rust 소개", "러스트는 빠르다")];
        let index = Index::build(docs);

        let stored = index.documents.get(&1).unwrap();
        assert_eq!(stored.title, "Rust 소개");
        assert_eq!(stored.url, "/1");
    }
}
