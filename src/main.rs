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
    // TODO: <https://datatracker.ietf.org/doc/html/rfc5322#section-3.4.1>
    let re = Regex::new(r#"^(?:([a-z0-9-_.]+)@)?(.+)$"#)?;
    let matches = re.captures(query).context("not match")?;
    let local_part = matches.get(1).map(|m| m.as_str()).unwrap_or("_");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bech32() -> anyhow::Result<()> {
        assert_eq!(
            format_bech32("3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d")?,
            "npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6"
        );
        Ok(())
    }

    #[test]
    fn test_parse_query() -> anyhow::Result<()> {
        assert_eq!(
            parse_query("bob@example.com")?,
            ("bob".to_owned(), "example.com".to_owned())
        );
        assert_eq!(
            parse_query("example.com")?,
            ("_".to_owned(), "example.com".to_owned())
        );
        Ok(())
    }
}
