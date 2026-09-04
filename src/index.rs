//! Inverted index + ranking. Filled in by #10 (build) and #11 (scoring).

use crate::tokenizer::tokenize;
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

impl Index {
    /// Builds an inverted index from a list of documents. (#10)
    pub fn build(docs: Vec<Document>) -> Index {
        let mut postings: HashMap<String, Vec<Posting>> = HashMap::new();
        let mut documents: HashMap<u32, Document> = HashMap::new();

        for doc in docs {
            let text = format!("{} {}", doc.title, doc.content);
            let tokens = tokenize(&text);

            let mut term_freq_map: HashMap<String, u32> = HashMap::new();
            for token in tokens {
                *term_freq_map.entry(token).or_insert(0) += 1;
            }

            for (token, freq) in term_freq_map {
                postings.entry(token).or_default().push(Posting {
                    doc_id: doc.id,
                    term_freq: freq,
                });
            }

            documents.insert(doc.id, doc);
        }

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

    #[test]
    fn test_build_and_lookup() {
        let docs = vec![
            Document {
                id: 1,
                title: "Rust 소개".into(),
                content: "러스트는 빠르다".into(),
                url: "/1".into(),
            },
            Document {
                id: 2,
                title: "Python 소개".into(),
                content: "파이썬은 쉽다".into(),
                url: "/2".into(),
            },
        ];
        let index = Index::build(docs);

        // "러스" 토큰은 문서 1에만 있어야 함
        let postings = index.postings.get("러스").unwrap();
        assert_eq!(postings.len(), 1);
        assert_eq!(postings[0].doc_id, 1);
    }
}
