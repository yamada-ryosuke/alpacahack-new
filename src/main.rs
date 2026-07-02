/// AlpacaHackのURLの構造体のあるモジュール
mod alpacahack_url;
/// 問題の情報を持つための構造体
mod challenge_info;
/// ユビキタス言語っぽいやつ
mod prelude;
/// 設定ファイルを管理するモジュール
mod config;

/// 各サブコマンドをまとめたモジュール
mod commands;


fn main() {
    commands::new::run();
}
