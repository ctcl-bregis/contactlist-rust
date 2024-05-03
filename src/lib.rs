// ContactList - CTCL 2023-2024
// File: src/lib.rs
// Purpose: Commonly used code
// Created: March 22, 2024
// Modified: May 1, 2024

use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;
use comrak::{markdown_to_html, Options};
use serde::{Deserialize, Serialize};

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HtmlTableKindInfo {
    #[serde(alias = "type")]
    // This value is added later
    ntype: Option<String>,
    col: String,
    width: u16
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HtmlTableKindInfotime {
    #[serde(rename = "type")]
    ntype: Option<String>,
    col: String,
    width: u16
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HtmlTableKindButton {
    link: String,
    icon: String,
    text: String,
    bsbtnstyle: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HtmlTableKindButtons {
    #[serde(rename = "type")]
    ntype: Option<String>,
    col: String,
    width: u16,
    buttons: Vec<HtmlTableKindButton>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum HtmlTableKind {
    #[serde(alias = "info")]
    Info(HtmlTableKindInfo),
    #[serde(alias = "infotime")]
    Infotime(HtmlTableKindInfotime),
    #[serde(alias = "buttons")]
    Buttons(HtmlTableKindButtons)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NavbarItem {
    pub name: String,
    pub text: String,
    pub link: String,
    pub float: String,
    #[serde(rename = "type")]
    // Added later
    pub ntype: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataSelect {
    col: String,
    cat: String,
    dropdown: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataString {
    col: String,
    cat: String,
    max: u16,
}

// Only difference between String and Text is that Text is displayed using the <textarea> tags
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataText {
    col: String,
    cat: String,
    max: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "datatype")]
pub enum DataType {
    #[serde(alias = "string")]
    String(DataString),
    #[serde(alias = "text")]
    Text(DataText),
    #[serde(alias = "select")]
    Select(DataSelect)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GlobalCfg {
    pub tablecats: HashMap<String, String>,
    pub table: Vec<DataType>,
    pub headers: IndexMap<String, String>,
    pub htmltable: Vec<HtmlTableKind>,
    pub dropdown: HashMap<String, IndexMap<String, String>>,
    pub navbar: Vec<NavbarItem>,
    pub strftime: String,
    pub sqlitedburl: String,
    pub bindip: String,
    pub bindport: u16,
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path))),
            _ => panic!("File read error when reading {}: {}", path.to_owned(), e),
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

pub fn mdpath2html(path: &str) -> Result<String, Error> {
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;
    let content = markdown_to_html(&read_file(path).unwrap_or_else(|_| panic!("File read error when reading: {}", path)), &comrak_options);

    Ok(content)
}

pub fn mkcontext(title: &str, globalcfg: GlobalCfg) -> Result<tera::Context, Error> {
    let mut ctx = tera::Context::new();

    let mut newhtmltable: Vec<HtmlTableKind> = Vec::new();
    for table in &globalcfg.htmltable {
        newhtmltable.push(match table {
            HtmlTableKind::Info(st) => HtmlTableKind::Info(HtmlTableKindInfo { ntype: Some("info".to_string()), col: st.col.clone(), width: st.width }),
            HtmlTableKind::Infotime(st) => HtmlTableKind::Infotime(HtmlTableKindInfotime { ntype: Some("infotime".to_string()), col: st.col.clone(), width: st.width }),
            HtmlTableKind::Buttons(st) => HtmlTableKind::Buttons(HtmlTableKindButtons { ntype: Some("info".to_string()), col: st.col.clone(), width: st.width, buttons: st.buttons.clone() }),
        });
    }

    ctx.insert("navbar", &globalcfg.navbar);
    ctx.insert("htmltable", &newhtmltable);
    ctx.insert("headers", &globalcfg.headers);
    ctx.insert("title", &title);

    Ok(ctx)
}

pub fn estconn(dburl: &str) -> SqliteConnection {
    SqliteConnection::establish(dburl).unwrap_or_else(|_| panic!("Error connecting to {}", dburl))
}