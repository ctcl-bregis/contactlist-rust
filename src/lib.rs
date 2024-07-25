// ContactList - CTCL 2023-2024
// File: src/lib.rs
// Purpose: Commonly used code
// Created: March 22, 2024
// Modified: June 16, 2024

use couch_rs::CouchDocument;
use couch_rs::document::{DocumentCollection, TypedCouchDocument};
use couch_rs::types::document::DocumentId;
use couch_rs::types::find::FindQuery;
use indexmap::IndexMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;
use std::result::Result;

// Config structs
#[derive(Serialize, Deserialize)]
pub struct ConfigHtmlTableButton {
    pub link: String,
    pub icon: String,
    pub text: String,
    pub bsbtnstyle: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfigHtmlTableItem {
    pub coltype: String,
    pub col: String,
    pub width: u8,
    pub buttons: Option<Vec<ConfigHtmlTableButton>>
}

#[derive(Serialize, Deserialize)]
pub struct ConfigNavbarItem {
    pub name: String,
    pub text: String,
    pub link: String,
    pub float: String,
    pub navtype: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub htmltable: Vec<ConfigHtmlTableItem>,
    pub navbar: Vec<ConfigNavbarItem>,
    pub strftime: String,
    pub bindip: String,
    pub bindport: u16
}

// Connection structs

#[derive(Serialize, Deserialize)]
pub struct Connection {
    pub cdburl: String,
    pub cdbuser: String,
    pub cdbpass: String,
    pub cdbdb: String
}

// Schema structs


#[derive(Serialize, Deserialize)]
pub struct SchemaField {
    #[serde(alias = "type")]
    pub ftype: String,
    pub title: String,
    pub cat: String,
    pub max: Option<u16>,
    pub dd: Option<String>,
    pub ddtitle: Option<String>,
    pub sttitle: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaDropdown {
    pub items: IndexMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub fields: IndexMap<String, SchemaField>,
    pub dropdown: HashMap<String, SchemaDropdown>
}

#[derive(Serialize, Deserialize)]
pub struct Combined {
    pub config: Config,
    pub connection: Connection,
    pub schema: Schema
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();

    if file_exists(path) {
        file.read_to_string(&mut buff).unwrap();
    }

    Ok(buff)
}

pub fn mkdir(path: &str) -> Result<(), Error> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir(path).expect("Could not create directory static/favicons/");
    }
    Ok(())
}

pub fn get_all_configs() -> Result<Combined, Error> {
    let config: Config = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();
    // TODO: connection.json is most likely to not exist at a given moment; add error handling for files
    let connection: Connection = serde_json::from_str(&read_file("config/connection.json").unwrap()).unwrap();
    let schema: Schema = serde_json::from_str(&read_file("config/schema.json").unwrap()).unwrap();

    Ok(
        Combined {
            config,
            connection,
            schema,
        }
    )
}