use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::problem_info::{ChallengeData, ChallengeInfo};

/// 問題プロジェクトを作成する。
pub(crate) fn create_project(
    alpacahack_dir: &Path,
    problem_info: ChallengeInfo,
) -> Result<PathBuf> {
    let challenge_dir = create_directory(alpacahack_dir, &problem_info.problem_name_with_kebab)
        .context("問題ディレクトリの作成に失敗しました。")?;
    println!(
        "問題ディレクトリを作成しました: {}",
        challenge_dir.display()
    );

    expand_file(&challenge_dir, problem_info.data).context("ファイルの展開に失敗しました。")?;
    println!("ファイルの展開が完了しました。");

    // 問題ディレクトリにmemo.mdを作成する。
    let memo_path = challenge_dir.join("memo.md");
    File::create(&memo_path).context("memo.mdの作成に失敗しました。")?;
    println!("memo.mdを作成しました: {}", memo_path.display());

    Ok(challenge_dir)
}

/// 問題プロジェクトのディレクトリを作成する。
fn create_directory(alpacahack_dir: &Path, downloaded_filename: &str) -> Result<PathBuf> {
    // ファイル名の末尾の.tar.gzを削除したものをディレクトリ名とする。
    let dirname = downloaded_filename.to_string().replace(".tar.gz", "");
    let dir_path = alpacahack_dir.join(dirname);
    fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}

/// ディレクトリの中にダウンロードしたファイルを展開する。
fn expand_file(problem_dir: &Path, downloaded_file: ChallengeData) -> Result<()> {
    // ダウンロードしたファイルを保存する。
    let downloaded_file_path = problem_dir.join(&downloaded_file.name);
    File::create(&downloaded_file_path)?.write_all(&downloaded_file.data)?;
    // ダウンロードしたファイルがtar.gzの場合、解凍する。
    if downloaded_file.name.ends_with(".tar.gz") {
        let tar_gz = File::open(&downloaded_file_path)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(problem_dir)?;
        // ダウンロードしたファイルを削除する。
        fs::remove_file(&downloaded_file_path)?;
    }
    Ok(())
}
