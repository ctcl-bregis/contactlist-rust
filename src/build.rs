// ContactList - CTCL 2023-2024
// File: src/build.rs
// Purpose: Builds files needed for the application
// Created: March 25, 2024
// Modified: July 24, 2024

use serde_json::value::Index;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Write};
use std::fs::{read, File};
use chrono::Utc;

// Field name regex: ^[a-z0-9]{4}$

//#[derive(Serialize, Deserialize, CouchDocument)]
//pub struct Contact {
//    #[serde(skip_serializing_if = "String::is_empty")]
//    pub _id: DocumentId,
//    #[serde(skip_serializing_if = "String::is_empty")]
//    pub _rev: String,
//
//}

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

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn mkdir(path: &str) -> Result<(), Error> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir(path).expect("Could not create directory static/favicons/");
    }
    Ok(())
}

pub fn cp(src: &str, dest: &str) -> Result<(), Error> {
    std::fs::copy(src, dest)?;
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

fn main() {
    // Step 1: Build schema?
    // Any point in making schemas? Likely the only reason why it was done with contactlist-python is to be able to use Django's ORM.
    // Database setup is done on app startup in main.rs

    // Step 2: Move files from node_modules

    cp("node_modules/bootstrap/dist/js/bootstrap.bundle.min.js", "static/bootstrap.js").unwrap();
    cp("node_modules/ckeditor5/build/ckeditor5-dll.js", "static/ckeditor.js").unwrap();
    cp("node_modules/jquery/dist/jquery.min.js", "static/jquery.js").unwrap();
    cp("node_modules/tablesorter/dist/js/jquery.tablesorter.combined.min.js", "static/tablesorter.js").unwrap();

    // Step 3: Build static files
    // Currently, "static/" likely would not exist as all of the files within it are excluded from commit with .gitignore
    mkdir("static/").unwrap();

    // Touch some grass
    let grass_options: Options = Options::default().style(OutputStyle::Compressed);


}