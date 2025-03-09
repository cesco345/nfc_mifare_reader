// ui/common.rs
use fltk::{
    button::Button,
    enums::FrameType,
    frame::Frame,
    group::{Group, Tabs},
    input::Input,
    menu::Choice,
    prelude::*,
    text::{TextBuffer, TextDisplay, TextEditor},
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::reader;
use crate::ui::converter;
use crate::batch;

pub fn create_reader_tab(tabs: &mut Tabs, keyboard_layout: Rc<RefCell<i32>>, card_data_buffer: Rc<RefCell<TextBuffer>>) {
    // Changed from y=50 to y=25 to align with tab bar
    let reader_tab = Group::new(0, 25, 800, 575, "Reader Mode");
    
    // Shared buffers
    let instructions_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    
    // Instructions section - adjusted y coordinates
    let mut instructions_frame = Frame::new(10, 35, 780, 100, "Instructions");
    instructions_frame.set_frame(FrameType::EngravedBox);
    
    let mut instructions_display = TextDisplay::new(20, 55, 760, 70, "");
    {
        let mut buffer = instructions_buffer.borrow_mut();
        buffer.set_text("Welcome to the Mifare Reader Utility!\n\n\
                       Present Mifare cards to the reader to capture their UIDs. UIDs will be automatically converted to human-readable format.");
        instructions_display.set_buffer(buffer.clone());
    }
    
    // Capture controls - adjusted y coordinates
    let mut capture_btn = Button::new(20, 145, 120, 30, "Start Capture");
    let mut clear_btn = Button::new(150, 145, 120, 30, "Clear Data");
    
    // Card data display - adjusted y coordinates
    let mut data_frame = Frame::new(10, 185, 780, 380, "Card Data");
    data_frame.set_frame(FrameType::EngravedBox);
    
    let mut card_data_display = TextDisplay::new(20, 205, 760, 350, "");
    {
        let buffer = card_data_buffer.borrow();
        card_data_display.set_buffer(buffer.clone());
    }
    
    let card_data_buffer_1 = card_data_buffer.clone();
    let kb_layout_for_capture = keyboard_layout.clone();
    capture_btn.set_callback(move |btn| {
        reader::start_capture(btn, card_data_buffer_1.clone(), kb_layout_for_capture.clone());
    });
    
    let card_data_buffer_2 = card_data_buffer.clone();
    clear_btn.set_callback(move |_| {
        if fltk::dialog::choice2(300, 300, "Are you sure you want to clear all captured data?", "Cancel", "Clear", "") == Some(1) {
            card_data_buffer_2.borrow_mut().set_text("");
        }
    });
    
    reader_tab.end();
    tabs.add(&reader_tab);
}

pub fn create_conversion_tab(tabs: &mut Tabs, keyboard_layout: Rc<RefCell<i32>>) {
    // Changed from y=50 to y=25 to align with tab bar
    let conversion_tab = Group::new(0, 25, 800, 575, "UID Conversion");
    
    // Adjusted all y coordinates by subtracting 25
    Frame::new(20, 45, 100, 30, "Enter Card UID:");
    let uid_input = Input::new(130, 45, 300, 30, "");
    let mut convert_btn = Button::new(450, 45, 100, 30, "Convert");
    
    Frame::new(20, 95, 740, 30, "Conversion Results:");
    
    // Result displays for conversion
    let hex_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let dec_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let mfg_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let format_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    
    Frame::new(20, 135, 200, 30, "Hexadecimal:");
    let mut hex_display = TextDisplay::new(230, 135, 530, 30, "");
    {
        let buffer = hex_buffer.borrow();
        hex_display.set_buffer(buffer.clone());
    }
    
    Frame::new(20, 175, 200, 30, "Decimal:");
    let mut dec_display = TextDisplay::new(230, 175, 530, 30, "");
    {
        let buffer = dec_buffer.borrow();
        dec_display.set_buffer(buffer.clone());
    }
    
    Frame::new(20, 215, 200, 30, "Manufacturer:");
    let mut mfg_display = TextDisplay::new(230, 215, 530, 30, "");
    {
        let buffer = mfg_buffer.borrow();
        mfg_display.set_buffer(buffer.clone());
    }
    
    Frame::new(20, 255, 200, 30, "Format Description:");
    let mut format_display = TextDisplay::new(230, 255, 530, 30, "");
    {
        let buffer = format_buffer.borrow();
        format_display.set_buffer(buffer.clone());
    }
    
    // Add instructions for keyboard encoding issues
    let mut kb_frame = Frame::new(20, 295, 740, 120, "");
    kb_frame.set_label(
        "Note about keyboard encoding: If you see special characters instead of numbers,\n\
        this utility will automatically convert them to the correct format based on selected keyboard layout.\n\n\
        Format codes explanation:\n\
        'e' = QWERTY keyboard, 'f' = AZERTY keyboard, 'h' = QUERTY keyboard, 'r' = reader specific format."
    );
    
    // Add keyboard layout selector
    Frame::new(20, 425, 180, 30, "Keyboard Layout:");
    
    let mut keyboard_choice = Choice::new(210, 425, 150, 30, "");
    keyboard_choice.add_choice("Auto-detect|Windows|Mac US|Mac International");
    keyboard_choice.set_value(0); // Default to Auto-detect
    
    let keyboard_layout_for_selector = keyboard_layout.clone();
    keyboard_choice.set_callback(move |c| {
        *keyboard_layout_for_selector.borrow_mut() = c.value();
    });
    
    // Create clones for use in callbacks
    let hex_buffer_clone = hex_buffer.clone();
    let dec_buffer_clone = dec_buffer.clone();
    let mfg_buffer_clone = mfg_buffer.clone();
    let format_buffer_clone = format_buffer.clone();
    let uid_input_clone = uid_input.clone();
    let keyboard_layout_for_convert = keyboard_layout.clone();
    
    convert_btn.set_callback(move |_| {
        converter::convert_uid(
            &uid_input_clone.value(), 
            *keyboard_layout_for_convert.borrow(),
            hex_buffer_clone.clone(),
            dec_buffer_clone.clone(),
            mfg_buffer_clone.clone(),
            format_buffer_clone.clone()
        );
    });
    
    conversion_tab.end();
    tabs.add(&conversion_tab);
}

pub fn create_batch_tab(tabs: &mut Tabs, keyboard_layout: Rc<RefCell<i32>>) {
    // Changed from y=50 to y=25 to align with tab bar
    let batch_tab = Group::new(0, 25, 800, 575, "Batch Conversion");
    
    // Adjusted all y coordinates by subtracting 25
    let mut batch_instructions = Frame::new(20, 45, 740, 50, "");
    batch_instructions.set_label("Paste multiple UIDs below, one per line. The application will convert all of them at once.");
    
    let batch_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    let batch_result_buffer = Rc::new(RefCell::new(TextBuffer::default()));
    
    // Use TextEditor instead of TextDisplay for editable input
    let mut batch_input = TextEditor::new(20, 105, 740, 150, "");
    batch_input.set_buffer(batch_buffer.borrow_mut().clone());
    batch_input.set_frame(FrameType::DownBox);
    batch_input.set_text_font(fltk::enums::Font::Courier);
    
    // Add clear input button for batch input
    let mut batch_clear_input_btn = Button::new(20, 265, 120, 30, "Clear Input");
    let batch_buffer_for_clear = batch_buffer.clone();
    batch_clear_input_btn.set_callback(move |_| {
        if fltk::dialog::choice2(300, 300, "Clear input data?", "Cancel", "Clear", "") == Some(1) {
            batch_buffer_for_clear.borrow_mut().set_text("");
        }
    });
    
    let mut batch_convert_btn = Button::new(350, 265, 120, 30, "Convert All");
    
    // Add clear results button
    let mut batch_clear_results_btn = Button::new(480, 265, 120, 30, "Clear Results");
    let batch_result_buffer_for_clear = batch_result_buffer.clone();
    batch_clear_results_btn.set_callback(move |_| {
        batch_result_buffer_for_clear.borrow_mut().set_text("");
    });
    
    let mut batch_results = TextDisplay::new(20, 305, 740, 210, "");
    batch_results.set_buffer(batch_result_buffer.borrow().clone());
    batch_results.set_text_font(fltk::enums::Font::Courier);
    
    let batch_buffer_clone = batch_buffer.clone();
    let batch_result_buffer_clone = batch_result_buffer.clone();
    let kb_layout_for_batch = keyboard_layout.clone();
    batch_convert_btn.set_callback(move |_| {
        batch::process_batch(
            &batch_buffer_clone.borrow().text(),
            *kb_layout_for_batch.borrow(),
            batch_result_buffer_clone.clone()
        );
    });
    
    batch_tab.end();
    tabs.add(&batch_tab);
}