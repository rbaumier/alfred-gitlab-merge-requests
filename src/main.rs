use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{env, io};

type MergeRequests = Vec<MergeRequest>;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
struct MergeRequest {
    title: String,
    web_url: String,
    state: String,
    created_at: DateTime<Utc>,
}

fn main() {
    let token = env::var("GITLAB_TOKEN").unwrap();
    let group = env::var("GITLAB_GROUP").unwrap();
    let pattern = env::args().nth(1).unwrap_or(String::from(""));
    let client = reqwest::Client::new();
    let merge_requests: MergeRequests = client
        .get(&format!(
            "https://gitlab.com/api/v4/groups/{}/merge_requests?scope=all",
            group
        ))
        .header("Private-Token", token)
        .send()
        .unwrap()
        .json()
        .unwrap();

    let alfred_items: Vec<_> = merge_requests
        .iter()
        .filter(|mr| {
            vec![&mr.title, &mr.state, &mr.web_url]
                .iter()
                .any(|value| value.to_lowercase().contains(&pattern.to_lowercase()))
        })
        .map(|mr| {
            let repo_name = mr
                .web_url
                .split(".com/")
                .skip(1)
                .collect::<String>()
                .split("/-/")
                .take(1)
                .collect::<String>();
            let state_emoji = match mr.state.as_str() {
                "opened" => "🚀",
                "merged" => "✅",
                "locked" => "🔒",
                _ => "🔐", // closed
            };
            let created_at = mr.created_at.format("%F %R");
            alfred::ItemBuilder::new(format!("{} {}", state_emoji, &mr.title))
                .subtitle(format!("[{}] {}", repo_name, created_at))
                .arg(&mr.web_url)
                .into_item()
        })
        .collect();

    alfred::json::write_items(io::stdout(), &alfred_items).unwrap();
}
