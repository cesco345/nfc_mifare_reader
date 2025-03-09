// utils.rs
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, TimeZone, Local};

/// Get current timestamps in both Unix and human-readable formats
pub fn get_timestamps() -> (String, String) {
    // Get current time
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    
    // Create both Unix and human-readable timestamps
    let unix_timestamp = format!("{}", secs);
    let datetime: DateTime<Local> = Local.timestamp_opt(secs as i64, 0).unwrap();
    let human_timestamp = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    
    (unix_timestamp, human_timestamp)
}

/// Process a UID into human-readable format
pub fn process_uid_for_display(uid: &str, keyboard_layout: i32) -> (String, String) {
    // First, handle keyboard encoding formats and normalize
    let decoded = match keyboard_layout {
        1 => decode_windows_format(uid),   // Windows
        2 => decode_mac_us_format(uid),    // Mac US
        3 => decode_mac_intl_format(uid),  // Mac International
        _ => {
            // Auto-detect: try to guess based on content
            if uid.contains('@') || uid.contains('!') || uid.contains('^') {
                // Likely Windows/standard encoding
                decode_windows_format(uid)
            } else if uid.contains('§') || uid.contains('±') {
                // Likely Mac with international chars
                decode_mac_intl_format(uid)
            } else {
                // Default to Mac US layout
                decode_mac_us_format(uid)
            }
        }
    };
    
    // Extract just the hex digits
    let clean_uid: String = decoded.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    
    if clean_uid.is_empty() {
        return ("Invalid format".to_string(), "Unknown".to_string());
    }
    
    // Format the hex UID with spaces for readability
    let formatted_hex = format_hex_uid(&clean_uid);
    
    // Determine manufacturer
    let manufacturer = identify_manufacturer(&clean_uid);
    
    (formatted_hex, manufacturer)
}

/// Format hex UID with spaces for better readability
pub fn format_hex_uid(hex_uid: &str) -> String {
    let chars: Vec<char> = hex_uid.chars().collect();
    let mut formatted = String::new();
    
    for (i, c) in chars.iter().enumerate() {
        formatted.push(*c);
        if (i + 1) % 2 == 0 && i < chars.len() - 1 {
            formatted.push(' ');
        }
    }
    
    formatted.to_uppercase()
}

/// Convert hexadecimal to decimal
pub fn hex_to_decimal(hex: &str) -> String {
    if hex.contains("Invalid") {
        return "N/A".to_string();
    }
    
    let clean_hex = hex.replace(" ", "");
    match u64::from_str_radix(&clean_hex, 16) {
        Ok(decimal) => decimal.to_string(),
        Err(_) => "Invalid hex value".to_string()
    }
}

/// Handle standard/Windows keyboard mapping
pub fn decode_windows_format(encoded_str: &str) -> String {
    if encoded_str.is_empty() {
        return String::new();
    }
    
    let mut decoded = String::new();
    
    for c in encoded_str.chars() {
        match c {
            '!' => decoded.push('1'),
            '@' => decoded.push('2'),
            '#' => decoded.push('3'),
            '$' => decoded.push('4'),
            '%' => decoded.push('5'),
            '^' => decoded.push('6'),
            '&' => decoded.push('7'),
            '*' => decoded.push('8'),
            '(' => decoded.push('9'),
            ')' => decoded.push('0'),
            'h' => decoded.push('h'),
            'd' => decoded.push('d'),
            'e' => decoded.push('e'),
            'r' => decoded.push('r'),
            '-' => decoded.push('-'),
            ' ' => decoded.push(' '),
            c if c.is_ascii_hexdigit() => decoded.push(c),
            _ => {}  // Skip other characters
        }
    }
    
    decoded
}

/// Handle Mac US keyboard mapping
pub fn decode_mac_us_format(encoded_str: &str) -> String {
    if encoded_str.is_empty() {
        return String::new();
    }
    
    let mut decoded = String::new();
    
    for c in encoded_str.chars() {
        match c {
            '!' => decoded.push('1'),
            '@' => decoded.push('2'),
            '#' => decoded.push('3'),
            '$' => decoded.push('4'),
            '%' => decoded.push('5'),
            '^' => decoded.push('6'),
            '&' => decoded.push('7'),
            '*' => decoded.push('8'),
            '(' => decoded.push('9'),
            ')' => decoded.push('0'),
            // Mac-specific mappings
            '¡' => decoded.push('1'),
            '™' => decoded.push('2'),
            '£' => decoded.push('3'),
            '¢' => decoded.push('4'),
            '∞' => decoded.push('5'),
            '§' => decoded.push('6'),
            '¶' => decoded.push('7'),
            '•' => decoded.push('8'),
            'ª' => decoded.push('9'),
            'º' => decoded.push('0'),
            // Format indicators
            'h' => decoded.push('h'),
            'd' => decoded.push('d'),
            'e' => decoded.push('e'),
            'r' => decoded.push('r'),
            '-' => decoded.push('-'),
            ' ' => decoded.push(' '),
            c if c.is_ascii_hexdigit() => decoded.push(c),
            _ => {}  // Skip other characters
        }
    }
    
    decoded
}

/// Handle Mac International keyboard mapping
pub fn decode_mac_intl_format(encoded_str: &str) -> String {
    if encoded_str.is_empty() {
        return String::new();
    }
    
    let mut decoded = String::new();
    
    for c in encoded_str.chars() {
        match c {
            // Standard shift+number mappings
            '!' => decoded.push('1'),
            '@' => decoded.push('2'),
            '#' => decoded.push('3'),
            '$' => decoded.push('4'),
            '%' => decoded.push('5'),
            '^' => decoded.push('6'),
            '&' => decoded.push('7'),
            '*' => decoded.push('8'),
            '(' => decoded.push('9'),
            ')' => decoded.push('0'),
            // Mac International specific mappings
            '¡' => decoded.push('1'),
            '™' => decoded.push('2'),
            '£' => decoded.push('3'),
            '¢' => decoded.push('4'),
            '∞' => decoded.push('5'),
            '§' => decoded.push('6'),
            '¶' => decoded.push('7'),
            '•' => decoded.push('8'),
            'ª' => decoded.push('9'),
            'º' => decoded.push('0'),
            '±' => decoded.push('='),
            '≠' => decoded.push('='),
            '€' => decoded.push('e'),
            // Additional international characters
            'ä' => decoded.push('a'),
            'á' => decoded.push('a'),
            'à' => decoded.push('a'),
            'é' => decoded.push('e'),
            'è' => decoded.push('e'),
            'í' => decoded.push('i'),
            'ì' => decoded.push('i'),
            'ó' => decoded.push('o'),
            'ò' => decoded.push('o'),
            'ú' => decoded.push('u'),
            'ù' => decoded.push('u'),
            // Format indicators
            'h' => decoded.push('h'),
            'd' => decoded.push('d'),
            'e' => decoded.push('e'),
            'r' => decoded.push('r'),
            '-' => decoded.push('-'),
            ' ' => decoded.push(' '),
            c if c.is_ascii_hexdigit() => decoded.push(c),
            _ => {}  // Skip other characters
        }
    }
    
    decoded
}

/// Identify manufacturer based on first byte of UID
pub fn identify_manufacturer(hex_uid: &str) -> String {
    if hex_uid.len() >= 2 {
        let manuf_code = &hex_uid[0..2].to_lowercase();
        match manuf_code.as_str() {
            "04" => "NXP Semiconductors".to_string(),
            "05" => "Infineon Technologies".to_string(),
            "16" => "Texas Instruments".to_string(),
            "21" => "EM Microelectronic-Marin SA".to_string(),
            "28" => "LEGIC Identsystems AG".to_string(),
            "29" => "Gemplus".to_string(),
            "33" => "Atmel".to_string(),
            "47" => "Orga Kartensysteme GmbH".to_string(),
            "49" => "Inside Technology".to_string(),
            "55" => "Tönnjes C.A.R.D. International".to_string(),
            "57" => "Giesecke & Devrient".to_string(),
            "75" => "HID Global".to_string(),
            "87" => "Identive".to_string(),
            "95" => "NXP MIFARE Classic".to_string(),
            "96" => "NXP MIFARE Plus".to_string(),
            "98" => "NXP MIFARE DESFire".to_string(),
            _ => "Unknown manufacturer".to_string(),
        }
    } else {
        "Unknown (UID too short)".to_string()
    }
}

/// Interpret format codes from the captured data
pub fn interpret_format_code(data: &str) -> String {
    // Look for format indicators
    if data.contains(" e") || data.contains("-e") {
        return "QWERTY keyboard layout".to_string();
    } else if data.contains(" f") || data.contains("-f") {
        return "AZERTY keyboard layout".to_string();
    } else if data.contains(" h") || data.contains("-h") {
        return "QUERTY keyboard layout".to_string();
    } else if data.contains(" r") || data.contains("-r") {
        return "Reader-specific format".to_string();
    } else if data.contains("format description") {
        return "Format description command".to_string();
    } else if data.contains("data format") {
        return "Data format specification".to_string();
    } else if data.contains("disable buzzer") {
        return "Reader configuration command".to_string();
    }
    
    // Check for patterns in your log data
    if data.contains("*h-!)d-e") {
        return "Card type 1 with QWERTY encoding".to_string();
    } else if data.contains("@h-#d-$h-%d-e") {
        return "Card type 2 with QWERTY encoding".to_string();
    } else if data.contains("*h-e") {
        return "Card type 3 with QWERTY encoding".to_string();
    }
    
    "Standard format".to_string()
}

/// Generate a report about a specific UID
pub fn generate_uid_report(uid: &str, keyboard_layout: i32) -> String {
    let (hex_uid, manufacturer) = process_uid_for_display(uid, keyboard_layout);
    let decimal = hex_to_decimal(&hex_uid);
    let format = interpret_format_code(uid);
    
    let (unix_time, human_time) = get_timestamps();
    
    format!(
        "UID Analysis Report\n\
        -------------------\n\
        Generated on: {} (Unix: {})\n\
        \n\
        Raw UID: {}\n\
        Hex UID: {}\n\
        Decimal UID: {}\n\
        Manufacturer: {}\n\
        Format: {}\n\
        \n\
        Keyboard layout used: {}\n",
        human_time,
        unix_time,
        uid,
        hex_uid,
        decimal,
        manufacturer,
        format,
        match keyboard_layout {
            0 => "Auto-detect",
            1 => "Windows",
            2 => "Mac US",
            3 => "Mac International",
            _ => "Unknown"
        }
    )
}

/// Check if a string contains any valid UID content
pub fn contains_uid_data(text: &str) -> bool {
    // Check for format indicators
    if text.contains("QWERTY") || text.contains("AZERTY") || text.contains("QUERTY") {
        return true;
    }
    
    // Check for common format codes
    if text.contains("-e") || text.contains("-h") || text.contains("-d") || text.contains("-r") {
        return true;
    }
    
    // Check for keyboard-encoded characters
    let special_chars = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '¡', '™', '£', '¢', '∞', '§'];
    for c in special_chars {
        if text.contains(c) {
            return true;
        }
    }
    
    // Check if it contains hexadecimal-only content
    let hex_only: String = text.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    
    if hex_only.len() >= 6 && hex_only.len() <= 28 {
        return true;
    }
    
    false
}

/// Extended mapping of card types based on UID characteristics
pub fn identify_card_type(hex_uid: &str) -> String {
    if hex_uid.is_empty() || hex_uid.contains("Invalid") {
        return "Unknown card type".to_string();
    }
    
    let len = hex_uid.replace(" ", "").len();
    
    match len {
        8 => "MIFARE Classic (4 byte UID)".to_string(),
        14 => "MIFARE Classic (7 byte UID)".to_string(),
        16 => "MIFARE DESFire (8 byte UID)".to_string(),
        20 => "MIFARE Plus (10 byte UID)".to_string(),
        4 => "Partial read/Single block ID".to_string(),
        _ if len < 8 => "Partial/Incomplete UID".to_string(),
        _ => format!("Non-standard card ({} byte UID)", len / 2)
    }
}