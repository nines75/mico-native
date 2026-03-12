use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write, stdin, stdout};
use std::path::Path;
use std::process::Command;

use heck::ToLowerCamelCase;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    path: String,
    should_check_wsl: bool,
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

        // WSLが起動していない場合はフィルターの読み取りをキャンセル
        if message.should_check_wsl && !is_wsl_running()? {
            write_message(&serde_json::to_vec(&json!({}))?)?;

            continue;
        }

        let files = [
            "ng-user-id",
            "ng-command",
            "ng-word",
            "ng-id",
            "ng-user-name",
            "ng-title",
        ];
        let base_path = Path::new(&message.path);
        let mut map = HashMap::new();

        for file in files {
            for extension in ["txt", "mico"] {
                let path = base_path.join(format!("{file}.{extension}"));

                if let Ok(content) = fs::read_to_string(path) {
                    map.insert(file.to_lower_camel_case(), content);
                }
            }
        }

        // jsonに変換して書き込み
        write_message(&serde_json::to_vec(&json!({"settings": map}))?)?;
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
