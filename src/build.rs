// ContactList - CTCL 2023-2024
// File: src/build.rs
// Purpose: Builds files needed for the application
// Created: March 25, 2024
// Modified: April 4, 2024

// touch grass
use grass::{Options, OutputStyle};
use std::collections::HashMap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Write};
use std::fs::{read, File};
use chrono::Utc;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DataSelect {
    col: String,
    cat: String,
    dropdown: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DataString {
    col: String,
    cat: String,
    max: u16,
}

// Only difference between String and Text is that Text is displayed using the <textarea> tags
#[derive(Deserialize, Serialize, Clone, Debug)]
struct DataText {
    col: String,
    cat: String,
    max: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "datatype")]
enum DataType {
    #[serde(alias = "string")]
    String(DataString),
    #[serde(alias = "text")]
    Text(DataText),
    #[serde(alias = "select")]
    Select(DataSelect)
}

#[derive(Deserialize, Serialize, Clone)]
struct GlobalCfg {
    // Just these fields are needed
    table: Vec<DataType>,
    dropdown: HashMap<String, IndexMap<String, String>>,
    headers: HashMap<String, String>,
    strftime: String
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path))),
            _ => panic!("Can't read from file: {}, err {}", path, e),
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

fn write_file(path: &str, content: &str) {
    let mut buffer = File::create(path).unwrap();
    buffer.write_all(content.as_bytes()).unwrap();
}

// Not so clever name for a function that copies a file only if the destination does not exist
fn copyifdestnotexists(src: String, dest: String) -> Result<(), Error> {
    if !std::path::Path::new(&src).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("file not found: {src}")));
    }

    if !std::path::Path::new(&dest).exists() {
        std::fs::copy(src, dest).unwrap();
    }

    Ok(())
}

fn mkdirifnotexists(dir: &str) -> Result<(), Error> {
    if !std::path::Path::new(&dir).exists() {
        match std::fs::create_dir(dir) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    } else {
        Ok(())
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();

    // Step 1.1 - Build models
    let timestamp = format!("{}", Utc::now().format(&globalcfg.strftime));

    let mut contacttable: String = String::new();
    for col in &globalcfg.table {
        let modelrow = match col {
            DataType::String(st) => format!("    pub {}: String,\n", st.col),
            DataType::Text(st) => format!("    pub {}: String,\n", st.col),
            DataType::Select(st) => format!("    pub {}: String,\n", st.col)
        };
        contacttable.push_str(&modelrow);
    }

    let schemafile = format!("// ContactList - CTCL 2023-2024
// File: src/models.rs
// Purpose: Database table struct definition
// Generated: {} - ContactList {}
// THIS FILE IS AUTOMATICALLY GENERATED

use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::contacts)]
pub struct Contact {{
    pub id: i32,
{}
}}
", timestamp, VERSION, contacttable);
    
    write_file("src/models.rs", &schemafile);

    // Step 1.2 - Build schema
    let mut contacttable: String = String::new();
    for col in &globalcfg.table {
        let modelrow = match col {
            DataType::String(st) => format!("        {} -> Text,\n", st.col),
            DataType::Text(st) => format!("        {} -> Text,\n", st.col),
            DataType::Select(st) => format!("        {} -> Text,\n", st.col)
        };
        contacttable.push_str(&modelrow);
    }

    let schemafile = format!("// ContactList - CTCL 2023-2024
// File: src/schema.rs
// Purpose: Database table schema definition
// Generated: {} - ContactList {}
// THIS FILE IS AUTOMATICALLY GENERATED

use diesel::table;

table! {{
    contacts (id) {{
        id -> Int4,
{}
    }}
}}
", timestamp, VERSION, contacttable);
    
    write_file("src/schema.rs", &schemafile);

    // Step 2 - Generate styling
    std::fs::create_dir_all("static/").unwrap();

    let grass_options: Options = Options::default()
    .style(OutputStyle::Compressed);

    let main_css = grass::from_string(read_file("config/styling/main.scss").unwrap(), &grass_options).unwrap();
    write_file("static/main.css", &main_css);

    let tstheme_css = grass::from_string(read_file("config/styling/ts_theme.scss").unwrap(), &grass_options).unwrap();
    write_file("static/ts_theme.css", &tstheme_css);

    // Step 3 - Copy over files from node_modules
    // unwrap() so it just panics with whatever error message the function returns
    mkdirifnotexists("static/").unwrap();
    copyifdestnotexists("node_modules/bootstrap/dist/js/bootstrap.min.js".to_string(), "static/bootstrap.min.js".to_string()).unwrap();
    copyifdestnotexists("node_modules/jquery/dist/jquery.min.js".to_string(), "static/jquery.min.js".to_string()).unwrap();
    copyifdestnotexists("node_modules/tablesorter/dist/js/jquery.tablesorter.min.js".to_string(), "static/jquery.tablesorter.min.js".to_string()).unwrap();
    copyifdestnotexists("node_modules/tablesorter/dist/js/jquery.tablesorter.widgets.min.js".to_string(), "static/jquery.tablesorter.widgets.min.js".to_string()).unwrap();
}