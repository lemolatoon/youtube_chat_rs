use anyhow::anyhow;
use regex::Regex;
struct ReqestOptions<'a> {
    live_id: &'a str,
    api_key: &'a str,
    client_version: &'a str,
    continuation: &'a str,
}

async fn get_options_from_live_page<'a>(data: &'a str) -> Result<ReqestOptions<'a>, anyhow::Error> {
    let live_id_regex =
        Regex::new(r#"<link rel="canonical" href="https:\/\/www.youtube.com\/watch\?v=(.+?)">"#)
            .unwrap();
    let live_id = match live_id_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("Live Stream was not found.")),
    };

    let replay_regex = Regex::new(r#"['"]isReplay['"]:\s*(true)"#).unwrap();
    match replay_regex.find(&data) {
        Some(_) => {}
        None => return Err(anyhow!("{live_id} is finished live.")),
    };

    let api_key_regex = Regex::new(r#"['"]INNERTUBE_API_KEY['"]:\s*['"](.+?)['"]"#).unwrap();
    let api_key = match api_key_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("{live_id} is finished live.")),
    };

    let client_version_regex = Regex::new(r#"['"]clientVersion['"]:\s*['"]([\d.]+?)['"]"#).unwrap();
    let client_version = match client_version_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("Client Version was not found.")),
    };

    let continuation_regex = Regex::new(r#"['"]continuation['"]:\s*['"](.+?)['"]"#).unwrap();
    let continuation = match continuation_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("Client Version was not found.")),
    };

    Ok(ReqestOptions {
        live_id,
        api_key,
        client_version,
        continuation,
    })
}
