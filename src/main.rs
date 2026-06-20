/// 問題の情報を持つための構造体
mod challenge_info;
/// 問題ページから問題の情報を取得する機能のモジュール
mod fetch;
/// 問題プロジェクトを作成する機能のモジュール
mod project;

use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result};
use reqwest::Url;

fn main() -> Result<()> {
    // 問題ページのURLを入力してもらう。
    let challenge_url = input_url().context("不正なURLです。")?;
    // AlpacaHackのディレクトリ名を取得する。
    let alpacahack_directory = get_alpacahack_directory()?;

    let challenge_dir = setup_challenge_project(&challenge_url, &alpacahack_directory)?;

    // VSCodeでディレクトリを開く。
    open_vscode(&challenge_dir).context("VSCodeでディレクトリを開けませんでした。")?;
    println!("VSCodeでディレクトリを開きました。");

    Ok(())
}

/// 指定した URL から問題データを取得し、作業ディレクトリに問題プロジェクトを作成する。
///
/// # 引数
/// - `file_url`: ダウンロード対象のファイルを指す `Url`。
/// - `alpacahack_directory`: ベースとなる作業ディレクトリの `Path`。
///
/// # 動作
/// 1. `file_url` から問題データを非同期で取得する（`fetch::fetch_problem_data` を呼ぶ）。
/// 2. 取得した問題情報をもとに、`alpacahack` 配下に問題用プロジェクトを作成する。
///
/// # 返り値
/// 作成した問題プロジェクトのディレクトリパス。
fn setup_challenge_project(challenge_url: &Url, alpacahack_directory: &Path) -> Result<PathBuf> {
    // 問題情報を取得する。
    let challenge_info = fetch::fetch_challenge_data(challenge_url)?;
    println!("問題情報を取得しました");
    println!("問題タイトル: {}", challenge_info.meta.title);

    // 問題プロジェクトを作成する。
    let challenge_dir = project::create_project(alpacahack_directory, challenge_info)?;
    println!("問題プロジェクトの作成が完了しました。");

    Ok(challenge_dir)
}

/// 問題ページのURLを入力してもらう。
fn input_url() -> Result<Url> {
    print!("問題ページのurl> ");
    io::stdout()
        .flush()
        .context("標準出力に失敗しました。")
        .unwrap();

    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .context("URLの入力に失敗しました")
        .unwrap();
    validate_domain(url.trim())
}

/// URLが https://alpacahack.com のものであることを確認する。
fn validate_domain(url: &str) -> Result<Url> {
    let url = Url::parse(url)?;
    let domain = url
        .domain()
        .ok_or(anyhow::anyhow!("ドメイン名がありません。"))?;
    if domain == "alpacahack.com" {
        Ok(url)
    } else {
        Err(anyhow::anyhow!("ドメイン名が正しくありません。"))
    }
}

/// alpacahackディレクトリのパスを取得する。
fn get_alpacahack_directory() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!(
        "ホームディレクトリが取得できませんでした。"
    ))?;
    let alpacahack_directory = home_dir.join("competitions").join("ctf").join("alpacahack");
    Ok(alpacahack_directory)
}

/// VSCodeで問題ディレクトリを開く。
fn open_vscode(challenge_dir: &Path) -> Result<()> {
    process::Command::new("code")
        .arg(challenge_dir)
        .spawn()?
        .wait()?;
    Ok(())
}

#[cfg(test)]
mod daily_alpacahack_test {
    use std::{fs, io::Read};

    use crate::challenge_info::ChallengeMeta;

    use super::*;
    use chrono::NaiveDate;
    use tempfile::tempdir;

    // 期待されるディレクトリ構成を表す構造体
    enum FsEntry {
        File {
            name: String,
        },
        Directory {
            name: String,
            children: Vec<FsEntry>,
        },
    }

    /// ディレクトリ構成が正しいことを確かめる関数
    fn assert_directory_structure(root: &Path, expected: &FsEntry) {
        match expected {
            FsEntry::File { name } => {
                let path = root.join(name);
                assert!(
                    path.is_file(),
                    "{}はファイルではありません。",
                    path.display()
                );
            }
            FsEntry::Directory { name, children } => {
                let dir = root.join(name);
                assert!(
                    dir.is_dir(),
                    "{}はディレクトリではありません。",
                    dir.display()
                );
                for child in children {
                    assert_directory_structure(&dir, child);
                }
            }
        }
    }

    /// challenge.tomlの中身が正しいことのテスト
    fn assert_challenge_toml(
        root: &Path,
        expected_challenge_toml: ChallengeMeta,
    ) {
        let mut challenge_toml = String::new();
        fs::File::open(root.join(&expected_challenge_toml.project_name).join("challenge.toml"))
            .unwrap()
            .read_to_string(&mut challenge_toml)
            .unwrap();
        let challenge_toml = toml::from_str::<ChallengeMeta>(&challenge_toml).unwrap();
        assert_eq!(challenge_toml, expected_challenge_toml);
    }

    /// 問題タイトルとファイル名が一致しているパターン
    #[test]
    fn test_emojify_matching() {
        let challenge_url = Url::parse("https://alpacahack.com/daily/challenges/emojify").unwrap();
        let _file_url = Url::parse("https://alpacahack-prod.s3.ap-northeast-1.amazonaws.com/5bad030b-a894-4111-900d-43332caf6bf6/emojify.tar.gz").unwrap();

        let dir = tempdir().unwrap();

        setup_challenge_project(&challenge_url, dir.path()).unwrap();

        use FsEntry::*;
        let expected_directory = Directory {
            name: "emojify".to_string(),
            children: vec![
                File {
                    name: "note.md".to_string(),
                },
                File {
                    name: "challenge.toml".to_string(),
                },
                Directory {
                    name: "emojify".to_string(),
                    children: vec![
                        Directory {
                            name: "backend".to_string(),
                            children: vec![
                                File {
                                    name: "index.js".to_string(),
                                },
                                File {
                                    name: "package-lock.json".to_string(),
                                },
                                File {
                                    name: "package.json".to_string(),
                                },
                            ],
                        },
                        Directory {
                            name: "frontend".to_string(),
                            children: vec![
                                File {
                                    name: "index.js".to_string(),
                                },
                                File {
                                    name: "index.html".to_string(),
                                },
                                File {
                                    name: "package-lock.json".to_string(),
                                },
                                File {
                                    name: "package.json".to_string(),
                                },
                            ],
                        },
                        Directory {
                            name: "secret".to_string(),
                            children: vec![
                                File {
                                    name: "index.js".to_string(),
                                },
                                File {
                                    name: "package-lock.json".to_string(),
                                },
                                File {
                                    name: "package.json".to_string(),
                                },
                            ],
                        },
                        File {
                            name: "compose.yaml".to_string(),
                        },
                        File {
                            name: "Dockerfile".to_string(),
                        },
                    ],
                },
            ],
        };

        assert_directory_structure(dir.path(), &expected_directory);

        // challenge.tomlの中身が正しいことのテスト
        assert_challenge_toml(
            dir.path(),
            ChallengeMeta {
                url: Url::parse("https://alpacahack.com/daily/challenges/emojify").unwrap(),
                released_at: NaiveDate::from_ymd_opt(2025, 12, 3).unwrap(),
                title: "Emojify".to_string(),
                project_name: "emojify".to_string(),
            },
        );
    }

    /// 問題タイトルとファイル名が一致していないパターン
    #[test]
    fn test_a_fact_of_ctf_mismatch() {
        let challenge_url =
            Url::parse("https://alpacahack.com/daily/challenges/a-fact-of-ctf").unwrap();
        let _file_url = Url::parse("https://alpacahack-prod.s3.ap-northeast-1.amazonaws.com/0a2e166c-fe68-4617-83d2-1ff98a4e5812/a-fact-of-CTF.tar.gz").unwrap();

        let dir = tempdir().unwrap();

        setup_challenge_project(&challenge_url, dir.path()).unwrap();

        use FsEntry::*;
        let expected_directory = Directory {
            name: "a-fact-of-ctf".to_string(),
            children: vec![
                File {
                    name: "note.md".to_string(),
                },
                File {
                    name: "challenge.toml".to_string(),
                },
                Directory {
                    name: "a-fact-of-CTF".to_string(),
                    children: vec![
                        File {
                            name: "chall.py".to_string(),
                        },
                        File {
                            name: "output.txt".to_string(),
                        },
                    ],
                },
            ],
        };
        assert_directory_structure(dir.path(), &expected_directory);

        // challenge.tomlの中身が正しいことのテスト
        assert_challenge_toml(
            dir.path(),
            ChallengeMeta {
                url: Url::parse("https://alpacahack.com/daily/challenges/a-fact-of-ctf").unwrap(),
                released_at: NaiveDate::from_ymd_opt(2025, 12, 2).unwrap(),
                title: "a fact of CTF".to_string(),
                project_name: "a-fact-of-ctf".to_string(),
            },
        );
    }

    /// ファイルが.tar.gzでないパターン
    #[test]
    fn test_non_tar_file() {
        let challenge_url =
            Url::parse("https://alpacahack.com/daily/challenges/read-assembly").unwrap();
        let _file_url = Url::parse("https://alpacahack-prod.s3.ap-northeast-1.amazonaws.com/d8a7fbf5-1a2f-4398-ab06-bc1422cf33c6/asm.txt").unwrap();

        let dir = tempdir().unwrap();

        setup_challenge_project(&challenge_url, dir.path()).unwrap();

        use FsEntry::*;
        let expected_directory = Directory {
            name: "read-assembly".to_string(),
            children: vec![
                File {
                    name: "note.md".to_string(),
                },
                File {
                    name: "challenge.toml".to_string(),
                },
                File {
                    name: "asm.txt".to_string(),
                },
            ],
        };
        assert_directory_structure(dir.path(), &expected_directory);
        
        // challenge.tomlの中身が正しいことのテスト
        assert_challenge_toml(
            dir.path(),
            ChallengeMeta {
                url: Url::parse("https://alpacahack.com/daily/challenges/read-assembly").unwrap(),
                released_at: NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
                title: "Read Assembly".to_string(),
                project_name: "read-assembly".to_string(),
            },
        );
    }

    /// ファイルがないパターン
    #[test]
    fn test_no_file() {
        let challenge_url =
            Url::parse("https://alpacahack.com/daily/challenges/alpacahack-2100").unwrap();

        let dir = tempdir().unwrap();

        setup_challenge_project(&challenge_url, dir.path()).unwrap();

        use FsEntry::*;
        let expected_directory = Directory {
            name: "alpacahack-2100".to_string(),
            children: vec![
                File {
                    name: "note.md".to_string(),
                },
                File {
                    name: "challenge.toml".to_string(),
                },
            ],
        };
        assert_directory_structure(dir.path(), &expected_directory);
        
        // challenge.tomlの中身が正しいことのテスト
        assert_challenge_toml(
            dir.path(),
            ChallengeMeta {
                url: Url::parse("https://alpacahack.com/daily/challenges/alpacahack-2100").unwrap(),
                released_at: NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
                title: "AlpacaHack 2100".to_string(),
                project_name: "alpacahack-2100".to_string(),
            },
        );
    }
}
