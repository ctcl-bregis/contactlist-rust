// ContactList - CTCL 2023-2024
// File: types.rs
// Purpose: Types available in the database configuration
// Created: October 9, 2024
// Modified: October 12, 2024

use std::collections::HashMap;

pub struct DatetimeField {
    timestamp: String
}

pub struct DropdownField {
    choice: String,
    #[serde(skip_serializing)]
    choices: HashMap<String, String>
}

//pub struct ImageField {

//}

pub struct StringField {
    content: String,
    #[serde(skip_serializing)]
    maxlength: u32
}

pub struct TextareaField {
    content: String,
    #[serde(skip_serializing)]
    maxlength: u32
}


pub enum FieldType {
    #[serde(alias = "")]
    Datetime(DatetimeField)
}
