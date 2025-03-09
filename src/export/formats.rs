// export/formats.rs
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

/// Export formats supported by the application
pub enum ExportFormat {
    CSV,
    JSON,
    Text,
}

/// Structure representing a card record
#[derive(Debug, Clone)]
pub struct CardRecord {
    pub timestamp: String,
    pub raw_uid: String,
    pub hex_uid: String,
    pub decimal_uid: String,
    pub manufacturer: String,
    pub format: String,
}

/// Export card data to a file
pub fn export_data(
    records: &[CardRecord], 
    format: ExportFormat, 
    filename: &str
) -> io::Result<String> {
    let content = match format {
        ExportFormat::CSV => generate_csv(records),
        ExportFormat::JSON => generate_json(records),
        ExportFormat::Text => generate_text(records),
    };
    
    let path = Path::new(filename);
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    
    Ok(format!("Data exported to {}", filename))
}

/// Generate CSV content from card records
fn generate_csv(records: &[CardRecord]) -> String {
    let mut csv = String::from("Timestamp,Raw UID,Hex UID,Decimal UID,Manufacturer,Format\n");
    
    for record in records {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            record.timestamp,
            record.raw_uid,
            record.hex_uid,
            record.decimal_uid,
            record.manufacturer,
            record.format
        ));
    }
    
    csv
}

/// Generate JSON content from card records
fn generate_json(records: &[CardRecord]) -> String {
    let mut json = String::from("[\n");
    
    for (i, record) in records.iter().enumerate() {
        json.push_str(&format!(
            "  {{\n    \"timestamp\": \"{}\",\n    \"raw_uid\": \"{}\",\n    \"hex_uid\": \"{}\",\n    \"decimal_uid\": \"{}\",\n    \"manufacturer\": \"{}\",\n    \"format\": \"{}\"\n  }}",
            record.timestamp,
            record.raw_uid,
            record.hex_uid,
            record.decimal_uid,
            record.manufacturer,
            record.format
        ));
        
        if i < records.len() - 1 {
            json.push_str(",\n");
        } else {
            json.push_str("\n");
        }
    }
    
    json.push_str("]\n");
    json
}

/// Generate plain text content from card records
fn generate_text(records: &[CardRecord]) -> String {
    let mut text = String::from("Mifare Reader Utility - Exported Data\n");
    text.push_str(&format!("Export Date: {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    for (i, record) in records.iter().enumerate() {
        text.push_str(&format!("Card #{}\n", i + 1));
        text.push_str(&format!("Timestamp: {}\n", record.timestamp));
        text.push_str(&format!("Raw UID: {}\n", record.raw_uid));
        text.push_str(&format!("Hex UID: {}\n", record.hex_uid));
        text.push_str(&format!("Decimal UID: {}\n", record.decimal_uid));
        text.push_str(&format!("Manufacturer: {}\n", record.manufacturer));
        text.push_str(&format!("Format: {}\n\n", record.format));
    }
    
    text
}

/// Parse data from text display and convert to card records
pub fn parse_display_text(text: &str) -> Vec<CardRecord> {
    let mut records = Vec::new();
    let mut lines = text.lines().peekable();
    
    while let Some(line) = lines.next() {
        // Look for lines that start with timestamps [numbers]
        if line.starts_with('[') && line.contains("] (") && line.contains("Raw UID:") {
            // Extract timestamp
            let timestamp = if let Some(end) = line.find(']') {
                line[1..end].to_string()
            } else {
                continue;
            };
            
            // Extract raw UID
            let raw_uid = if let Some(start) = line.find("Raw UID: ") {
                let start = start + "Raw UID: ".len();
                line[start..].to_string()
            } else {
                continue;
            };
            
            // Extract other fields from subsequent lines
            let mut hex_uid = String::new();
            let mut decimal_uid = String::new();
            let mut manufacturer = String::new();
            let mut format = String::new();
            
            // Try to read the next lines for additional data
            while let Some(next_line) = lines.peek() {
                if next_line.trim().starts_with("→ Hex:") {
                    hex_uid = next_line
                        .trim()
                        .trim_start_matches("→ Hex:")
                        .trim()
                        .to_string();
                    lines.next();
                } else if next_line.trim().starts_with("→ Decimal:") {
                    decimal_uid = next_line
                        .trim()
                        .trim_start_matches("→ Decimal:")
                        .trim()
                        .to_string();
                    lines.next();
                } else if next_line.trim().starts_with("→ Manufacturer:") {
                    manufacturer = next_line
                        .trim()
                        .trim_start_matches("→ Manufacturer:")
                        .trim()
                        .to_string();
                    lines.next();
                } else if next_line.trim().starts_with("→ Format:") {
                    format = next_line
                        .trim()
                        .trim_start_matches("→ Format:")
                        .trim()
                        .to_string();
                    lines.next();
                } else {
                    // Not a continuation line, break
                    break;
                }
            }
            
            // Add the record if we have the minimum data
            if !hex_uid.is_empty() {
                records.push(CardRecord {
                    timestamp,
                    raw_uid,
                    hex_uid,
                    decimal_uid,
                    manufacturer,
                    format,
                });
            }
        }
    }
    
    records
}