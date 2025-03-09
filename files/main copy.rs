use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType, CallbackTrigger},
    frame::Frame,
    group::{Group, Tabs},
    input::Input,
    prelude::*,
    text::{TextBuffer, TextDisplay, TextEditor},
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, TimeZone, Local};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "Mifare Reader Utility");
    
    // Create tabs for different functions
    let tabs = Tabs::new(10, 10, 780, 580, "");
    
    // 1. Reader Mode Tab
    let reader_tab = Group::new(10, 35, 780, 555, "Reader Mode");
    
    // Shared buffers
    let card_data_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let instructions_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    
    // Instructions section
    let mut instructions_frame = Frame::new(10, 10, 760, 100, "Instructions");
    instructions_frame.set_frame(FrameType::EngravedBox);
    
    let mut instructions_display = TextDisplay::new(20, 30, 740, 70, "");
    {
        let mut buffer = instructions_buffer.borrow_mut();
        buffer.set_text("Welcome to the Mifare Reader Utility!\n\n\
                       Present Mifare cards to the reader to capture their UIDs. UIDs will be automatically converted to human-readable format.");
        instructions_display.set_buffer(buffer.clone());
    }
    
    // Capture controls
    let mut capture_btn = Button::new(20, 120, 120, 30, "Start Capture");
    let mut clear_btn = Button::new(150, 120, 120, 30, "Clear Data");
    
    // Card data display
    let mut data_frame = Frame::new(10, 160, 760, 380, "Card Data");
    data_frame.set_frame(FrameType::EngravedBox);
    
    let mut card_data_display = TextDisplay::new(20, 180, 740, 350, "");
    {
        let buffer = card_data_buffer.borrow();
        card_data_display.set_buffer(buffer.clone());
    }
    
    let card_data_buffer_1 = card_data_buffer.clone();
    capture_btn.set_callback(move |btn| {
        if btn.label() == "Start Capture" {
            btn.set_label("Stop Capture");
            
            // Create a capture window
            let mut capture_wind = Window::new(300, 300, 500, 200, "Card Capture");
            capture_wind.set_color(Color::White);
            
            Frame::new(20, 20, 460, 40, "Present cards to the reader\nCard data will appear here:").set_label_size(14);
            
            let mut capture_input = Input::new(20, 80, 460, 30, "");
            capture_input.set_trigger(CallbackTrigger::EnterKey);
            
            let card_buffer = card_data_buffer_1.clone();
            
            // Function to process card data
            capture_input.set_callback(move |inp| {
                let data = inp.value();
                if !data.is_empty() {
                    if !data.contains("config") && !data.contains("Buz") {
                        // Format timestamp 
                        let now = SystemTime::now();
                        let duration = now.duration_since(UNIX_EPOCH).unwrap();
                        let secs = duration.as_secs();
                        
                        // Create both Unix and human-readable timestamps
                        let unix_timestamp = format!("{}", secs);
                        let datetime: DateTime<Local> = Local.timestamp_opt(secs as i64, 0).unwrap();
                        let human_timestamp = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                        
                        // Process the UID for human-readable format
                        let (hex_uid, manufacturer) = process_uid_for_display(&data);
                        
                        // Calculate decimal value for human readability
                        let mut decimal_value = "N/A".to_string();
                        if !hex_uid.contains("Invalid") {
                            let clean_hex = hex_uid.replace(" ", "");
                            if let Ok(decimal) = u64::from_str_radix(&clean_hex, 16) {
                                decimal_value = decimal.to_string();
                            }
                        }
                        
                        // Interpret format
                        let format_desc = interpret_format_code(&data);
                        
                        // Create a more detailed record
                        let record = format!(
                            "[{}] ({}) Raw UID: {}\n    → Hex: {}\n    → Decimal: {}\n    → Manufacturer: {}\n    → Format: {}\n\n", 
                            unix_timestamp,
                            human_timestamp, 
                            data, 
                            hex_uid,
                            decimal_value, 
                            manufacturer,
                            format_desc
                        );
                        
                        // Add to the display
                        let mut buffer = card_buffer.borrow_mut();
                        let current = buffer.text();
                        buffer.set_text(&format!("{}{}", current, record));
                    }
                    inp.set_value("");
                }
            });
            
            // Make the input focus automatically
            capture_input.take_focus().unwrap();
            
            capture_wind.end();
            capture_wind.show();
            
            let mut btn_clone = btn.clone();
            // Set window close callback
            capture_wind.set_callback(move |w| {
                w.hide();
                btn_clone.set_label("Start Capture");
            });
            
        } else {
            btn.set_label("Start Capture");
            // No need to worry about closing windows - they'll close themselves
        }
    });
    
    let card_data_buffer_2 = card_data_buffer.clone();
    clear_btn.set_callback(move |_| {
        if fltk::dialog::choice2(300, 300, "Are you sure you want to clear all captured data?", "Cancel", "Clear", "") == Some(1) {
            card_data_buffer_2.borrow_mut().set_text("");
        }
    });
    
    reader_tab.end();
    
    // 2. UID Conversion Tab
    let conversion_tab = Group::new(10, 35, 780, 555, "UID Conversion");
    
    Frame::new(20, 50, 100, 30, "Enter Card UID:");
    let mut uid_input = Input::new(130, 50, 300, 30, "");
    let mut convert_btn = Button::new(450, 50, 100, 30, "Convert");
    
    Frame::new(20, 100, 740, 30, "Conversion Results:");
    
    // Result displays for conversion
    let hex_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let dec_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let mfg_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let format_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    
    Frame::new(20, 140, 200, 30, "Hexadecimal:");
    let mut hex_display = TextDisplay::new(230, 140, 530, 30, "");
    {
        let buffer = hex_buffer.borrow();
        hex_display.set_buffer(buffer.clone());
    }
    
    Frame::new(20, 180, 200, 30, "Decimal:");
    let mut dec_display = TextDisplay::new(230, 180, 530, 30, "");
    {
        let buffer = dec_buffer.borrow();
        dec_display.set_buffer(buffer.clone());
    }
    
    Frame::new(20, 220, 200, 30, "Manufacturer:");
    let mut mfg_display = TextDisplay::new(230, 220, 530, 30, "");
    {
        let buffer = mfg_buffer.borrow();
        mfg_display.set_buffer(buffer.clone());
    }
    
    Frame::new(20, 260, 200, 30, "Format Description:");
    let mut format_display = TextDisplay::new(230, 260, 530, 30, "");
    {
        let buffer = format_buffer.borrow();
        format_display.set_buffer(buffer.clone());
    }
    
    let hex_buffer_clone = hex_buffer.clone();
    let dec_buffer_clone = dec_buffer.clone();
    let mfg_buffer_clone = mfg_buffer.clone();
    let format_buffer_clone = format_buffer.clone();
    
    convert_btn.set_callback(move |_| {
        let uid = uid_input.value().trim().to_string();
        if uid.is_empty() {
            return;
        }
        
        // Process the UID
        let (hex_result, manufacturer) = process_uid_for_display(&uid);
        hex_buffer_clone.borrow_mut().set_text(&hex_result);
        mfg_buffer_clone.borrow_mut().set_text(&manufacturer);
        
        // Try to convert to decimal
        if !hex_result.contains("Invalid") {
            let clean_hex = hex_result.replace(" ", "");
            match u64::from_str_radix(&clean_hex, 16) {
                Ok(decimal) => {
                    dec_buffer_clone.borrow_mut().set_text(&decimal.to_string());
                },
                Err(_) => {
                    dec_buffer_clone.borrow_mut().set_text("Invalid hex value");
                }
            }
        } else {
            dec_buffer_clone.borrow_mut().set_text("Invalid format");
        }
        
        // Interpret format code
        let format_description = interpret_format_code(&uid);
        format_buffer_clone.borrow_mut().set_text(&format_description);
    });
    
    // Add instructions for the Windows character encoding issue
    let mut windows_frame = Frame::new(20, 300, 740, 180, "");
    windows_frame.set_label(
        "Note about Windows encoding: If you see special characters like !@#$%^&*() instead of numbers 1234567890,\n\
        this utility will automatically convert them to the correct format.\n\n\
        Format codes explanation:\n\
        'e' = QWERTY keyboard, 'f' = AZERTY keyboard, 'h' = QUERTY keyboard, 'r' = reader specific format.\n\n\
        Card number conversion: \n\
        ! = 1, @ = 2, # = 3, $ = 4, % = 5, ^ = 6, & = 7, * = 8, ( = 9, ) = 0"
    );
    
    conversion_tab.end();
    
    // 3. Add a new tab for Batch Conversion
    let batch_tab = Group::new(10, 35, 780, 555, "Batch Conversion");
    
    let mut batch_instructions = Frame::new(20, 50, 740, 50, "");
    batch_instructions.set_label("Paste multiple UIDs below, one per line. The application will convert all of them at once.");
    
    let batch_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let batch_result_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    
    // Use TextEditor instead of TextDisplay for editable input
    let mut batch_input = TextEditor::new(20, 110, 740, 150, "");
    batch_input.set_buffer(batch_buffer.borrow_mut().clone());
    batch_input.set_frame(FrameType::DownBox);
    batch_input.set_text_font(fltk::enums::Font::Courier);
    
    // Add clear input button for batch input
    let mut batch_clear_input_btn = Button::new(20, 270, 120, 30, "Clear Input");
    let batch_buffer_for_clear = batch_buffer.clone();
    batch_clear_input_btn.set_callback(move |_| {
        if fltk::dialog::choice2(300, 300, "Clear input data?", "Cancel", "Clear", "") == Some(1) {
            batch_buffer_for_clear.borrow_mut().set_text("");
        }
    });
    
    let mut batch_convert_btn = Button::new(350, 270, 120, 30, "Convert All");
    
    // Add clear results button
    let mut batch_clear_results_btn = Button::new(480, 270, 120, 30, "Clear Results");
    let batch_result_buffer_for_clear = batch_result_buffer.clone();
    batch_clear_results_btn.set_callback(move |_| {
        batch_result_buffer_for_clear.borrow_mut().set_text("");
    });
    
    let mut batch_results = TextDisplay::new(20, 310, 740, 230, "");
    batch_results.set_buffer(batch_result_buffer.borrow().clone());
    batch_results.set_text_font(fltk::enums::Font::Courier);
    
    let batch_buffer_clone = batch_buffer.clone();
    let batch_result_buffer_clone = batch_result_buffer.clone();
    batch_convert_btn.set_callback(move |_| {
        let text = batch_buffer_clone.borrow().text();
        let lines: Vec<&str> = text.split('\n').collect();
        
        let mut results = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            let (hex_uid, manufacturer) = process_uid_for_display(line);
            let format_desc = interpret_format_code(line);
            
            // Calculate decimal value for human readability
            let mut decimal_value = "N/A".to_string();
            if !hex_uid.contains("Invalid") {
                let clean_hex = hex_uid.replace(" ", "");
                if let Ok(decimal) = u64::from_str_radix(&clean_hex, 16) {
                    decimal_value = decimal.to_string();
                }
            }
            
            results.push_str(&format!("UID #{}: {}\n", i + 1, line));
            results.push_str(&format!("   → Hex: {}\n", hex_uid));
            results.push_str(&format!("   → Decimal: {}\n", decimal_value));
            results.push_str(&format!("   → Manufacturer: {}\n", manufacturer));
            results.push_str(&format!("   → Format: {}\n\n", format_desc));
        }
        
        batch_result_buffer_clone.borrow_mut().set_text(&results);
    });
    
    batch_tab.end();
    
    tabs.end();
    wind.end();
    wind.show();
    
    app.run().unwrap();
}

/// Process a UID into human-readable format
fn process_uid_for_display(uid: &str) -> (String, String) {
    // First, handle Windows encoded formats and normalize
    let decoded = decode_windows_format(uid);
    
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
fn format_hex_uid(hex_uid: &str) -> String {
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

/// Handle Windows character encoding where numbers are represented by special characters
fn decode_windows_format(encoded_str: &str) -> String {
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

/// Identify manufacturer based on first byte of UID
fn identify_manufacturer(hex_uid: &str) -> String {
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
fn interpret_format_code(data: &str) -> String {
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