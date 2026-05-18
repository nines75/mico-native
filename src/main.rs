use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write, stdin, stdout};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum Message {
    #[serde(rename_all = "camelCase")]
    ImportLocalFilter {
        path: String,
        should_check_wsl: bool,
    },

    #[serde(rename_all = "camelCase")]
    SaveBackup {
        path: String,
        should_check_interval: bool,
        interval_threshold: u64,
        backup: HashMap<String, Value>,
    },
}

// 約1MB
const MAX_RESPONSE_SIZE: usize = 1_000_000;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
    }
}

fn run() -> Result<()> {
    loop {
        // 入力を読み取ってデシリアライズ
        let message: Message = serde_json::from_slice(&read_message()?)?;

        match message {
            Message::ImportLocalFilter {
                path,
                should_check_wsl,
            } => {
                // WSLが起動していない場合はフィルターの読み取りをキャンセル
                if should_check_wsl && !is_wsl_running()? {
                    write_message(&serde_json::to_vec(&json!({}))?)?;

                    continue;
                }

                let path = Path::new(&path);
                let mut map = HashMap::new();

                if let Ok(content) = fs::read_to_string(path) {
                    map.insert("manualFilter", content);
                }

                // jsonに変換して書き込み
                write_message(&serde_json::to_vec(&json!({"settings": map}))?)?;
            }
            Message::SaveBackup {
                path,
                should_check_interval,
                interval_threshold,
                backup,
            } => {
                let path = Path::new(&path);
                if !path.is_dir() {
                    return Err(anyhow!("指定されたディレクトリは存在しません"));
                }

                let path = path.join("mico-backup.json");
                if should_check_interval && path.is_file() {
                    let interval = path.metadata()?.modified()?.elapsed()?;
                    if interval <= Duration::from_hours(interval_threshold) {
                        write_message(&serde_json::to_vec(&json!({"status": "skipped"}))?)?;

                        continue;
                    }
                }

                // バックアップ書き出し
                fs::write(path, serde_json::to_string(&backup)?)?;

                write_message(&serde_json::to_vec(&json!({ "status": "completed" }))?)?;
            }
        }
    }
}

fn read_message() -> Result<Vec<u8>> {
    // ヘッダーを読み込む(4バイトで固定)
    let mut header = [0u8; 4];
    stdin().read_exact(&mut header)?;

    // リトルエンディアンとしてu32に変換
    // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_messaging#app_side
    let len = u32::from_le_bytes(header) as usize;

    // メッセージ本文を読み込む
    let mut body = vec![0u8; len];
    stdin().read_exact(&mut body)?;

    Ok(body)
}

fn write_message(body: &[u8]) -> Result<()> {
    if body.len() > MAX_RESPONSE_SIZE {
        return Err(anyhow!("レスポンスの大きさが上限を超えています"));
    }

    let len = body.len() as u32;
    let header = len.to_le_bytes();

    // バイト列をそのまま書き込む
    stdout().write_all(&header)?;
    stdout().write_all(body)?;
    stdout().flush()?;

    Ok(())
}

fn is_wsl_running() -> Result<bool> {
    let output = Command::new("wsl")
        .args(["--list", "--running", "--quiet"])
        .output()?;

    // 出力が空でないなら起動している
    Ok(output.status.success() && !output.stdout.is_empty())
}
