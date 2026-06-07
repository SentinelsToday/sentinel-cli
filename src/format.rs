use serde::Serialize;

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Json,
    Text,
}

impl OutputFormat {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "json" => Ok(Self::Json),
            "text" => Ok(Self::Text),
            other => Err(format!("unknown output format: {other}")),
        }
    }
}

pub fn render<T: Serialize>(value: &T, fmt: OutputFormat) -> String {
    match fmt {
        OutputFormat::Json => serde_json::to_string_pretty(value).unwrap_or_default(),
        OutputFormat::Text => render_text(value),
    }
}

fn render_text<T: Serialize>(value: &T) -> String {
    let v = serde_json::to_value(value).unwrap_or_default();
    let mut out = String::new();
    walk(&v, "", &mut out);
    out.trim_end().to_string()
}

fn walk(v: &serde_json::Value, prefix: &str, out: &mut String) {
    match v {
        serde_json::Value::Object(map) => {
            for (k, val) in map {
                let next = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                if val.is_object() || val.is_array() {
                    walk(val, &next, out);
                } else {
                    out.push_str(&format!("{next}: {}\n", display_scalar(val)));
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let next = format!("{prefix}[{i}]");
                if val.is_object() || val.is_array() {
                    walk(val, &next, out);
                } else {
                    out.push_str(&format!("{next}: {}\n", display_scalar(val)));
                }
            }
        }
        scalar => out.push_str(&format!("{prefix}: {}\n", display_scalar(scalar))),
    }
}

fn display_scalar(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip() {
        let v = serde_json::json!({"a": 1, "b": [2, 3]});
        let s = render(&v, OutputFormat::Json);
        let back: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn text_flattens_nested_keys() {
        let v = serde_json::json!({"trust": {"score": 90, "level": "verified"}, "robot": "r1"});
        let s = render(&v, OutputFormat::Text);
        assert!(s.contains("trust.score: 90"));
        assert!(s.contains("trust.level: verified"));
        assert!(s.contains("robot: r1"));
    }
}
