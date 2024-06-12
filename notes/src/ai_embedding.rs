use std::{collections::HashMap, env};

use anyhow::Context;
use reqwest::Url;
use serde_json::{json, Value};

use crate::repository::Note;

pub async fn search(query: &str) -> anyhow::Result<HashMap<usize, f64>> {
    let ai_server = env::var("AI_ENGINE")
        .context("connecting to ai embedding engine")
        .unwrap();
    let client = reqwest::Client::new();
    let body = HashMap::from([("query", query)]);
    let ai_server = Url::parse(&ai_server)?.join("/search")?;
    client
        .get(ai_server)
        .json(&body)
        .send()
        .await?
        .json::<HashMap<usize, f64>>()
        .await
        .context("search request error!")
}

pub async fn add(notes: &Vec<Note>) -> anyhow::Result<()> {
    let ai_server = env::var("AI_ENGINE")
        .context("connecting to ai embedding engine")
        .unwrap();
    let client = reqwest::Client::new();
    let ai_server = Url::parse(&ai_server)?.join("/add")?;
    for note in notes {
        let body = HashMap::from([
            ("id", json!(note.id)),
            ("sentence", json!(String::from(note))),
        ]);
        let payload: Value = serde_json::to_value(body)?;
        _ = client
            .post(ai_server.clone())
            .json(&payload)
            .send()
            .await?
            .text()
            .await
            .context("add request error!");
    }
    Ok(())
}
