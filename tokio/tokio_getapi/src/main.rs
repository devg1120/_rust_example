use anyhow::{Context, Result};
use futures::future::try_join_all;
use reqwest::header;
use serde_json::Value;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// リポジトリの API にアクセスし、スター数を返します
async fn get_star_count(client: &reqwest::Client, repo: String) -> Result<u64> {
    let resp: Value = client
        .get(&format!("https://api.github.com/repos/{}", repo))
        .send()
        .await? // API にアクセスします
        .json()
        .await?; // JSON をパースします
    let count = resp
        .get("stargazers_count")
        .context("GitHub API error: stargazers_count is not found")?
        .as_u64()
        .context("GitHub API error: stargazers_count is not an integer")?;
    Ok(count)
}

#[tokio::main]
async fn main() -> Result<()> {
    // HTTP request header と user agent を設定します
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let client = reqwest::Client::builder()
        .user_agent("rust reqwest")
        .default_headers(headers)
        .build()?;
    // 複数の非同期タスクで共有するので、Arc で包みます
    let client = Arc::new(client);

    // 調べたいリポジトリのリストです
    let repos = vec![
        "rust-lang-nursery/failure".to_string(),
        "rust-lang-nursery/lazy-static.rs".to_string(),
        "rust-lang/libc".to_string(),
        "bitflags/bitflags".to_string(),
        "rust-lang/log".to_string(),
    ];

    // spawn した非同期タスクのハンドラです
    let handles: Vec<JoinHandle<Result<u64>>> = repos
        .iter()
        .map(|repo| {
            // tokio::spawn は 'static を要求するため、clone しておきます
            let client = client.clone();
            let repo = repo.clone();

            // client と repo を move します
            tokio::spawn(async move {
                // 非同期タスクの実行結果は Result<u64> です
                get_star_count(&client, repo).await
            })
        })
        .collect::<Vec<_>>();

    // repos に対応するスター数です
    let stars: Vec<u64> = try_join_all(handles) // Vec<Result<T>> を Result<Vec<T>> に変換してくれます
        .await?
        .into_iter()
        .collect::<Result<Vec<u64>>>()?; // Vec<Result<T>> を Result<Vec<T>> に変換します

    for (repo, star) in repos.iter().zip(stars) {
        println!("{} has {} stars", repo, star);
    }

    Ok(())
}


