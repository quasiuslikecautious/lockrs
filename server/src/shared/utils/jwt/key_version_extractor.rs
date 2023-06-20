use base64::{engine::general_purpose, Engine as _};
use uuid::Uuid;

pub fn extract_key_version_from_token(token: &str) -> Option<Uuid> {
    let parts = token.split('.').collect::<Vec<&str>>();

    if parts.len() != 3 {
        return None;
    }

    // let _header = parts[0];
    let payload = parts[1];
    // let _signature = parts[2];

    let version_str = extract_field_from_payload(payload, "rev")?;
    Uuid::parse_str(&version_str).ok()
}

pub fn extract_field_from_payload(payload: &str, field_name: &str) -> Option<String> {
    let decoded_payload = general_purpose::URL_SAFE_NO_PAD.decode(payload).ok()?;
    let payload_str = String::from_utf8(decoded_payload).ok()?;
    let payload_json = serde_json::from_str::<serde_json::Value>(&payload_str).ok()?;
    let field = payload_json.get(field_name)?;
    let field_str = field.as_str()?;
    Some(field_str.to_string())
}
