use std::{collections::HashMap, str::FromStr};

use anyhow::Context;
use bech32::ToBase32;
use regex::Regex;
use reqwest::Url;
use secp256k1::XOnlyPublicKey;

#[derive(Debug, clap::Parser)]
struct Args {
    query: String,
}

async fn fetch_public_key(local_part: &str, domain: &str) -> anyhow::Result<String> {
    #[derive(Debug, serde::Deserialize)]
    struct Json {
        names: HashMap<String, String>,
    }
    let json = reqwest::get(&format!(
        "https://{domain}/.well-known/nostr.json?name={local_part}"
    ))
    .await?
    .json::<Json>()
    .await?;
    let public_key = json
        .names
        .get(local_part)
        .context("name not found")?
        .to_string();
    Ok(public_key)
}

fn format_bech32(public_key_in_hex: &str) -> anyhow::Result<String> {
    let public_key = XOnlyPublicKey::from_str(public_key_in_hex)?;
    let bech32 = bech32::encode(
        "npub",
        public_key.serialize().to_base32(),
        bech32::Variant::Bech32,
    )?;
    Ok(bech32)
}

fn parse_query(query: &str) -> anyhow::Result<(String, String)> {
    // TODO: support `example.com` => `_@example.com`
    let re = Regex::new(r#"^(?:([a-z0-9-_.]+)@)?(.+)$"#)?;
    let matches = re.captures(query).context("not match")?;
    let local_part = matches.get(1).context("local-part not found")?.as_str();
    let domain = matches.get(2).context("domain not found")?.as_str();
    let url = Url::parse(&format!("https://{domain}"))?;
    let domain = url.domain().context("domain is invalid")?;
    Ok((local_part.to_owned(), domain.to_owned()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    let (local_part, domain) = parse_query(&args.query)?;
    let public_key = fetch_public_key(&local_part, &domain).await?;
    let bech32 = format_bech32(&public_key)?;
    println!("{bech32}");
    Ok(())
}
