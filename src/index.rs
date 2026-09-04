//! 역색인 + 랭킹. #10(빌드), #11(스코어링)이 여기 채워진다.

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
    // TODO: postings, documents 등 내부 상태 — 이슈 #10 참고
}

impl Index {
    /// 문서 목록으로 역색인을 만든다. (#10)
    pub fn build(_docs: Vec<Document>) -> Index {
        todo!("역색인 빌드 — 이슈 #10 참고")
    }

    /// 쿼리와 관련도 높은 순으로 문서를 정렬해 반환한다. (#11)
    pub fn score(&self, _query: &str) -> Vec<ScoredDoc> {
        todo!("BM25 스코어링 — 이슈 #11 참고")
    }
}
