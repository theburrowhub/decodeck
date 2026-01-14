//! Recursive scanning of JSON/XML for encoded content

use crate::encoding::{detect::detect_encoding, DetectionConfidence, EncodingType};
use crate::error::DecodeckError;
use serde_json::Value as JsonValue;

/// Result of scanning a structured document
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Found encoded values with their paths and decoded content
    pub findings: Vec<Finding>,
    /// Total values scanned
    pub values_scanned: usize,
    /// Format detected (json/xml)
    pub format: String,
}

/// A single finding of encoded content
#[derive(Debug, Clone)]
pub struct Finding {
    /// JSON path or XPath to the value
    pub path: String,
    /// Original encoded value
    pub original: String,
    /// Decoded value (as string if valid UTF-8, otherwise hex representation)
    pub decoded: String,
    /// Detected encoding type
    pub encoding: EncodingType,
    /// Detection confidence
    pub confidence: DetectionConfidence,
    /// Whether decoded content is valid UTF-8
    pub is_text: bool,
}

/// Scan JSON content for encoded values
pub fn scan_json(input: &str) -> Result<ScanResult, DecodeckError> {
    let value: JsonValue = serde_json::from_str(input).map_err(|e| DecodeckError::DecodeFailed {
        message: format!("Invalid JSON: {}", e),
    })?;

    let mut findings = Vec::new();
    let mut values_scanned = 0;

    scan_json_value(&value, "$", &mut findings, &mut values_scanned);

    Ok(ScanResult {
        findings,
        values_scanned,
        format: "json".to_string(),
    })
}

/// Recursively scan a JSON value
fn scan_json_value(
    value: &JsonValue,
    path: &str,
    findings: &mut Vec<Finding>,
    scanned: &mut usize,
) {
    match value {
        JsonValue::String(s) => {
            *scanned += 1;
            if let Some(finding) = try_decode_value(s, path) {
                findings.push(finding);
            }
        }
        JsonValue::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let item_path = format!("{}[{}]", path, i);
                scan_json_value(item, &item_path, findings, scanned);
            }
        }
        JsonValue::Object(obj) => {
            for (key, val) in obj {
                let key_path = format!("{}.{}", path, key);
                scan_json_value(val, &key_path, findings, scanned);
            }
        }
        _ => {} // Skip numbers, bools, nulls
    }
}

/// Scan XML content for encoded values
pub fn scan_xml(input: &str) -> Result<ScanResult, DecodeckError> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(input);
    reader.config_mut().trim_text(true);

    let mut findings = Vec::new();
    let mut values_scanned = 0;
    let mut path_stack: Vec<String> = vec![];
    let mut current_path = String::from("/");

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                path_stack.push(name.clone());
                current_path = format!("/{}", path_stack.join("/"));

                // Check attributes
                for attr in e.attributes().flatten() {
                    let attr_name = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let attr_value = String::from_utf8_lossy(&attr.value).to_string();
                    let attr_path = format!("{}/@{}", current_path, attr_name);

                    values_scanned += 1;
                    if let Some(finding) = try_decode_value(&attr_value, &attr_path) {
                        findings.push(finding);
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if !text.trim().is_empty() {
                    values_scanned += 1;
                    if let Some(finding) = try_decode_value(&text, &current_path) {
                        findings.push(finding);
                    }
                }
            }
            Ok(Event::End(_)) => {
                path_stack.pop();
                current_path = if path_stack.is_empty() {
                    "/".to_string()
                } else {
                    format!("/{}", path_stack.join("/"))
                };
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(DecodeckError::DecodeFailed {
                    message: format!("Invalid XML: {}", e),
                });
            }
            _ => {}
        }
    }

    Ok(ScanResult {
        findings,
        values_scanned,
        format: "xml".to_string(),
    })
}

/// Try to decode a string value and return a Finding if successful
fn try_decode_value(value: &str, path: &str) -> Option<Finding> {
    let trimmed = value.trim();

    // Skip very short values or values that don't look encoded
    if trimmed.len() < 4 {
        return None;
    }

    // Detect encoding
    let info = detect_encoding(trimmed);

    // Only report findings with medium or high confidence
    if info.confidence == DetectionConfidence::Low {
        return None;
    }

    // Try to decode
    let decoder = info.encoding_type.decoder();
    match decoder.decode(trimmed) {
        Ok(decoded) => {
            let (decoded_str, is_text) = match String::from_utf8(decoded.clone()) {
                Ok(s) => (s, true),
                Err(_) => {
                    // Show as hex for binary content
                    let hex: String = decoded.iter().map(|b| format!("{:02x}", b)).collect();
                    (format!("(binary: {})", hex), false)
                }
            };

            Some(Finding {
                path: path.to_string(),
                original: trimmed.to_string(),
                decoded: decoded_str,
                encoding: info.encoding_type,
                confidence: info.confidence,
                is_text,
            })
        }
        Err(_) => None,
    }
}

/// Auto-detect format and scan
pub fn scan_auto(input: &str) -> Result<ScanResult, DecodeckError> {
    let trimmed = input.trim();

    // Try JSON first
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if let Ok(result) = scan_json(input) {
            return Ok(result);
        }
    }

    // Try XML
    if trimmed.starts_with('<') {
        if let Ok(result) = scan_xml(input) {
            return Ok(result);
        }
    }

    // Default to JSON attempt
    scan_json(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_json_simple() {
        let json = r#"{"data": "SGVsbG8gV29ybGQ=", "name": "test"}"#;
        let result = scan_json(json).unwrap();
        assert_eq!(result.findings.len(), 0); // Base64 has low confidence without markers
    }

    #[test]
    fn test_scan_json_hex() {
        let json = r#"{"data": "0x48656c6c6f", "name": "test"}"#;
        let result = scan_json(json).unwrap();
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].decoded, "Hello");
        assert_eq!(result.findings[0].encoding, EncodingType::Hex);
    }

    #[test]
    fn test_scan_json_nested() {
        let json = r#"{"outer": {"inner": "0x48656c6c6f"}, "arr": ["0x576f726c64"]}"#;
        let result = scan_json(json).unwrap();
        assert_eq!(result.findings.len(), 2);
    }

    #[test]
    fn test_scan_xml() {
        let xml = r#"<root><data>0x48656c6c6f</data></root>"#;
        let result = scan_xml(xml).unwrap();
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].path, "/root/data");
    }

    #[test]
    fn test_scan_xml_attributes() {
        let xml = r#"<root data="0x48656c6c6f">content</root>"#;
        let result = scan_xml(xml).unwrap();
        assert_eq!(result.findings.len(), 1);
        assert!(result.findings[0].path.contains("@data"));
    }

    #[test]
    fn test_scan_auto() {
        let json = r#"{"x": "0x48656c6c6f"}"#;
        let result = scan_auto(json).unwrap();
        assert_eq!(result.format, "json");

        let xml = r#"<r><d>0x48656c6c6f</d></r>"#;
        let result = scan_auto(xml).unwrap();
        assert_eq!(result.format, "xml");
    }
}
