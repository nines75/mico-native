use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write, stdin, stdout};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use Response::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum Request {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "status")]
enum Response {
    Completed {
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Value>,
    },
    Failed {
        error: String,
    },
    Skipped,
}

// 約1MB
const MAX_RESPONSE_SIZE: usize = 1_000_000;

fn main() {
    if let Err(error) = run() {
        write_response(Failed {
            error: format!("{error:?}"),
        })
        .unwrap();
    }
}

fn run() -> Result<()> {
    loop {
        // 入力を読み取ってデシリアライズ
        let request: Request = serde_json::from_slice(&read_request()?)?;

        match request {
            Request::ImportLocalFilter {
                path,
                should_check_wsl,
            } => {
                // WSLが起動していない場合はフィルターの読み取りをキャンセル
                if should_check_wsl && !is_wsl_running()? {
                    write_response(Skipped)?;

                    continue;
                }

                let path = Path::new(&path);
                let content = fs::read_to_string(path)?;

                // jsonに変換して書き込み
                write_response(Completed {
                    data: Some(json!({ "manualFilter": content })),
                })?;
            }
            Request::SaveBackup {
                path,
                should_check_interval,
                interval_threshold,
                backup,
            } => {
                let path = Path::new(&path);

                let path = path.join("mico-backup.json");
                if should_check_interval && path.is_file() {
                    let interval = path.metadata()?.modified()?.elapsed()?;
                    if interval <= Duration::from_hours(interval_threshold) {
                        write_response(Skipped)?;

                        continue;
                    }
                }

                // バックアップ書き出し
                fs::write(path, serde_json::to_string(&backup)?)?;

                write_response(Completed { data: None })?;
            }
        }
    }
}

fn read_request() -> Result<Vec<u8>> {
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

fn write_response(response: Response) -> Result<()> {
    let body = serde_json::to_vec(&response)?;

    if body.len() > MAX_RESPONSE_SIZE {
        return Err(anyhow!("レスポンスの大きさが上限を超えています"));
    }

    let len = body.len() as u32;
    let header = len.to_le_bytes();

    // バイト列をそのまま書き込む
    stdout().write_all(&header)?;
    stdout().write_all(&body)?;
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
