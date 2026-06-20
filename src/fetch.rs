use anyhow::Result;
use reqwest::Url;

/// ファイルをダウンロードする。
pub async fn download(url: &Url) -> Result<bytes::Bytes> {
    Ok(reqwest::get(url.as_str()).await?.bytes().await?)
}

/// URLからファイル名を取得する。
pub fn get_filename(url: &Url) -> Result<String> {
    let filename = url
        .path_segments()
        .ok_or(anyhow::anyhow!("URLのパスがありません。"))?
        .next_back()
        .ok_or(anyhow::anyhow!("URLのパスが空です。"))?;
    Ok(filename.to_owned())
}