// use reqwest::Url;

pub struct ChallengeInfo {
    // /// 問題のURL
    // pub problem_url: Url,
    // /// スペースを含む(おそらく正式な)問題の名前
    // pub problem_name_with_space: String,
    /// URLやディレクトリ名に使われるkebab-caseの問題の名前
    pub problem_name_with_kebab: String,
    /// 配布されるデータ
    pub data: ChallengeData,
}

/// 問題ページで配布されるファイル
pub struct ChallengeData {
    /// ファイル名
    pub name: String,
    /// データ
    pub data: bytes::Bytes,
}
