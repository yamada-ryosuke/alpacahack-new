use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};
use anyhow::Result;

/// 問題ディレクトリを作成する。
pub(crate) fn create_directory(alpacahack_directory: &Path, downloaded_filename: &str) -> Result<PathBuf> {
    // ファイル名の末尾の.tar.gzを削除したものをディレクトリ名とする。
    let dirname = downloaded_filename.to_string().replace(".tar.gz", "");
    let dir_path = alpacahack_directory.join(dirname);
    fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}

/// ディレクトリの中にファイルを展開する。
pub(crate) fn expand_file(dir: &Path, downloaded_filename: &str, downloaded_data: &[u8]) -> Result<()> {
    // ダウンロードしたファイルを保存する。
    let downloaded_file_path = dir.join(downloaded_filename);
    File::create(&downloaded_file_path)?.write_all(downloaded_data)?;
    // ダウンロードしたファイルがtar.gzの場合、解凍する。
    if downloaded_filename.ends_with(".tar.gz") {
        let tar_gz = File::open(&downloaded_file_path)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(dir)?;
        // ダウンロードしたファイルを削除する。
        fs::remove_file(&downloaded_file_path)?;
    }
    Ok(())
}

