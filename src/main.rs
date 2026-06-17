use std::{error::Error, io::{self, Write}};

use anyhow::Result;
use reqwest::{Response, Url};


#[tokio::main]
async fn main() -> Result<()> {
    // URLを入力してもらう。
    let url = input_url()?;
    // ファイルをダウンロードする
    let file = download(&url).await?;
    // 

    Ok(())
}

/// URLを入力してもらう。
fn input_url() -> Result<Url> {
    print!("url> ");
    io::stdout().flush()?;

    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    validate_domain(&url.trim().to_owned())
}

/// ファイルをダウンロードする。
async fn download(url: &Url) -> Result<bytes::Bytes> {
    Ok(reqwest::get(url.as_str()).await?.bytes().await?)
}

/// URLのドメインがalpacahack.comかを判定する。
fn validate_domain(url: &str) -> Result<Url> {
    let url = Url::parse(url)?;
    let domain = url
        .domain()
        .ok_or(anyhow::anyhow!("ドメイン名がありません。"))?;
    if domain == "alpacahack.com" {
        Ok(url)
    } else {
        Err(anyhow::anyhow!("ドメイン名がalpacahack.comではありません。"))
    }
}