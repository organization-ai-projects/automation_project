pub fn parse_env_pair_cli(raw: &str) -> Result<(String, String), String> {
    let mut split = raw.splitn(2, '=');
    let key = split.next().unwrap_or_default().trim();
    let value = split.next();
    if key.is_empty() || value.is_none() {
        return Err(format!("Invalid env pair '{}', expected KEY=VALUE", raw));
    }
    Ok((key.to_string(), value.unwrap_or_default().to_string()))
}
