//! Inverted index + ranking. Filled in by #10 (build) and #11 (scoring).

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
    // TODO: postings, documents, etc. — see issue #10
}

impl Index {
    /// Builds an inverted index from a list of documents. (#10)
    pub fn build(_docs: Vec<Document>) -> Index {
        todo!("build the inverted index — see issue #10")
    }

    /// Returns documents sorted by relevance to the query. (#11)
    pub fn score(&self, _query: &str) -> Vec<ScoredDoc> {
        todo!("BM25 scoring — see issue #11")
    }
}
